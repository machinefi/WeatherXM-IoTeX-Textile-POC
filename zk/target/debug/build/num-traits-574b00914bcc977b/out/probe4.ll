; ModuleID = 'probe4.5788c81e02566123-cgu.0'
source_filename = "probe4.5788c81e02566123-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_e1a66dc9d6605b5c4e39f5bcdf7bf5ff = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/90e321d82a0a9c3d0e3f180d4d17541b729072e0/library/core/src/num/mod.rs" }>, align 1
@alloc_42dade5320ac2f6bc7c257f3c822a615 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e1a66dc9d6605b5c4e39f5bcdf7bf5ff, [16 x i8] c"K\00\00\00\00\00\00\00y\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal unnamed_addr constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17h5bf76d72975826a7E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h55c676cb38714216E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h66f29befd47315b5E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_42dade5320ac2f6bc7c257f3c822a615) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h55c676cb38714216E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h66f29befd47315b5E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.76.0-nightly (90e321d82 2023-12-02)"}
