use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "thumbv7em-none-eabihf".to_string(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),

        options: TargetOptions {
            families: vec!["unix".to_string()],
            os: "espidf".to_string(),
            env: "newlib".to_string(),
            abi: "eabihf".to_string(),
            vendor: "espressif".to_string(),
            // `+vfp4` is the lowest common denominator between the Cortex-M4 (vfp4-16) and the
            // Cortex-M7 (vfp5)
            // `-d32` both the Cortex-M4 and the Cortex-M7 only have 16 double-precision registers
            // available
            // `-fp64` The Cortex-M4 only supports single precision floating point operations
            // whereas in the Cortex-M7 double precision is optional
            //
            // Reference:
            // ARMv7-M Architecture Reference Manual - A2.5 The optional floating-point extension
            features: "+vfp4,-d32,-fp64".to_string(),
            max_atomic_width: Some(32),
            ..super::thumb_base::opts()
        },
    }
}
