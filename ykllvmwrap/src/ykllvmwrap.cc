// LLVM-related C++ code wrapped in the C ABI for calling from Rust.

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

#include <llvm/DebugInfo/Symbolize/Symbolize.h>
#include <llvm/ExecutionEngine/ExecutionEngine.h>
#include <llvm/ExecutionEngine/MCJIT.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/Verifier.h>
#include <llvm/IRReader/IRReader.h>
#include <llvm/Support/SourceMgr.h>
#include <llvm/Support/TargetSelect.h>
#include <llvm/Transforms/Utils/ValueMapper.h>

#include <dlfcn.h>
#include <err.h>
#include <link.h>
#include <stdlib.h>
#include <string.h>

#include "memman.cc"

using namespace llvm;
using namespace llvm::symbolize;

#define TRACE_FUNC_NAME "__yk_compiled_trace"
#define YKTRACE_START "__yktrace_start_tracing"
#define YKTRACE_STOP "__yktrace_stop_tracing"

extern "C" LLVMSymbolizer *__yk_llvmwrap_symbolizer_new() {
  return new LLVMSymbolizer;
}

extern "C" void __yk_llvmwrap_symbolizer_free(LLVMSymbolizer *Symbolizer) {
  delete Symbolizer;
}

// Finds the name of a code symbol from a virtual address.
// The caller is responsible for freeing the returned (heap-allocated) C string.
extern "C" char *
__yk_llvmwrap_symbolizer_find_code_sym(LLVMSymbolizer *Symbolizer,
                                       const char *Obj, uint64_t Off) {
  object::SectionedAddress Mod{Off, object::SectionedAddress::UndefSection};
  auto LineInfo = Symbolizer->symbolizeCode(Obj, Mod);
  if (auto Err = LineInfo.takeError()) {
    return NULL;
  }

  // OPTIMISE_ME: get rid of heap allocation.
  return strdup(LineInfo->FunctionName.c_str());
}

// Load an LLVM module from an address.
std::unique_ptr<Module> load_module(LLVMContext &Context, char *Ptr,
                                    size_t Len) {
  auto Sf = StringRef(Ptr, Len);
  auto Mb = MemoryBufferRef(Sf, "");
  SMDiagnostic Error;
  auto M = parseIR(Mb, Error, Context);
  if (!M)
    errx(EXIT_FAILURE, "Can't load module.");
  return M;
}

// Compile a module in-memory and return a pointer to its function.
extern "C" void *compile_module(Module *M) {
  InitializeNativeTarget();
  InitializeNativeTargetAsmPrinter();
  InitializeNativeTargetAsmParser();

  // FIXME Remember memman or allocated memory pointers so we can free the
  // latter when we're done with the trace.
  auto memman = new MemMan();

  auto MPtr = std::unique_ptr<Module>(M);
  ExecutionEngine *EE =
      EngineBuilder(std::move(MPtr))
          .setMemoryManager(std::unique_ptr<MCJITMemoryManager>(memman))
          .create();
  EE->finalizeObject();

  if (EE->hasError())
    errx(EXIT_FAILURE, "Couldn't compile trace: %s",
         EE->getErrorMessage().c_str());

  return (void *)EE->getFunctionAddress(TRACE_FUNC_NAME);
}

// Compile an IRTrace to executable code in memory.
//
// The trace to compile is passed in as two arrays of length Len. Then each
// (FuncName[I], BBs[I]) pair identifies the LLVM block at position `I` in the
// trace.
//
// Returns a pointer to the compiled function.
extern "C" void *__ykllvmwrap_irtrace_compile(char *FuncNames[], size_t BBs[],
                                              size_t Len, char *SecPtr,
                                              size_t SecSize) {
  LLVMContext Context;
  auto DstMod = new Module("", Context);

  // Set up new module for the trace.
  llvm::FunctionType *FType =
      llvm::FunctionType::get(Type::getVoidTy(Context), false);
  llvm::Function *DstFunc = llvm::Function::Create(
      FType, Function::InternalLinkage, TRACE_FUNC_NAME, DstMod);
  DstFunc->setCallingConv(CallingConv::C);
  auto DstBB = BasicBlock::Create(Context, "bb0", DstFunc);
  llvm::IRBuilder<> Builder(Context);
  Builder.SetInsertPoint(DstBB);

  auto SrcMod = load_module(Context, SecPtr, SecSize);

  llvm::ValueToValueMapTy VMap;
  bool Tracing = false;

  // Iterate over the PT trace and stitch together all traced blocks.
  for (size_t Idx = 0; Idx < Len; Idx++) {
    auto FuncName = FuncNames[Idx];

    // Get a traced function so we can extract blocks from it.
    Function *F = SrcMod->getFunction(FuncName);
    if (!F)
      errx(EXIT_FAILURE, "can't find function %s", FuncName);

    // Skip to the correct block.
    auto It = F->begin();
    std::advance(It, BBs[Idx]);
    BasicBlock *BB = &*It;
    // Iterate over all instructions within this block and copy them over
    // to our new module.
    for (auto I = BB->begin(); I != BB->end(); I++) {
      if (isa<CallInst>(I)) {
        Function *CF = cast<CallInst>(&*I)->getCalledFunction();
        if (CF->getName() == YKTRACE_START) {
          Tracing = true;
          I++; // FIXME Better way of removing result of
               // __yktrace_start_tracing.
          continue;
        } else if (CF->getName() == YKTRACE_STOP) {
          // FIXME Remove argument setup before __yktrace_stop_tracing call.
          Tracing = false;
        }
      }

      if (!Tracing)
        continue;

      // FIXME For any variable defined outside of the trace, we realise
      // dummy storage space. Eventually, such variables should be inputs
      // to the trace. This hack prevents the remapper from choking when
      // it can't find the definition of an instruction operand.
      if ((llvm::isa<llvm::StoreInst>(I)) && (VMap[&*I] == nullptr)) {
        Value *DestOp = I->getOperand(1);
        Value *SrcOp = I->getOperand(0);
        Instruction *Alloca =
            Builder.CreateAlloca(SrcOp->getType(), (unsigned)0);
        VMap[DestOp] = Alloca;
      } else if ((llvm::isa<llvm::LoadInst>(I)) && (VMap[&*I] == nullptr)) {
        Value *SrcOp = I->getOperand(0);
        Instruction *Alloca =
            Builder.CreateAlloca(SrcOp->getType(), (unsigned)0);
        VMap[SrcOp] = Alloca;
      }

      if (llvm::isa<llvm::BranchInst>(I)) {
        // FIXME Replace all branch instruction with guards.
        continue;
      }

      // Copy instruction over into the IR trace. Since the instruction
      // operands still reference values in the original bitcode, remap
      // the operands to point to new values within the IR trace.
      auto NewInst = &*I->clone();
      llvm::RemapInstruction(NewInst, VMap, RF_NoModuleLevelChanges);
      VMap[&*I] = NewInst;
      Builder.Insert(NewInst);
    }
  }
  Builder.CreateRetVoid();
#ifdef DEBUG
  llvm::verifyModule(*DstMod, &llvm::errs());
#endif

  // Compile IR trace and return a pointer to its function.
  return compile_module(DstMod);
}
