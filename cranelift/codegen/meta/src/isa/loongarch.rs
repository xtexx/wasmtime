use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::SettingGroupBuilder;

pub(crate) fn define() -> TargetIsa {
    let mut settings = SettingGroupBuilder::new("loongarch");

    let has_32s = settings.add_bool(
        "has_32s",
        "Has support for basic 32-bit integer instructions.",
        "",
        false,
    );
    let has_64bit = settings.add_bool(
        "has_64bit",
        "Has support for basic 64-bit integer instructions.",
        "",
        false,
    );
    let has_f = settings.add_bool(
        "has_f",
        "Has support for single-precision floating point.",
        "",
        false,
    );
    let has_d = settings.add_bool(
        "has_d",
        "Has support for double-precision floating point.",
        "",
        false,
    );
    let _has_lsx = settings.add_bool(
        "has_lsx",
        "Has support for LSX.",
        "Loongson SIMD Extension",
        false,
    );
    let _has_lasx = settings.add_bool(
        "has_lasx",
        "Has support for LASX.",
        "Loongson Advanced SIMD Extension",
        false,
    );

    settings.add_preset(
        "baseline64",
        "A 64-bit baseline preset for LoongArch.",
        preset!(has_32s && has_64bit && has_f && has_d),
    );

    TargetIsa::new("loongarch", settings.build())
}
