#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(crate) enum Target {
    x86_64_unknown_linux_gnu,
    i686_unknown_linux_gnu,
    powerpc_unknown_linux_gnu,
    powerpc64_unknown_linux_gnu,
    mips_unknown_linux_gnu,
    arm_linux_androideabi,
}

impl Into<&'static str> for Target {
    fn into(self: Target) -> &'static str {
        match self {
            Self::x86_64_unknown_linux_gnu => "x86_64-unknown-linux-gnu",
            Self::i686_unknown_linux_gnu => "i686-unknown-linux-gnu",
            Self::powerpc64_unknown_linux_gnu => "powerpc64-unknown-linux-gnu",
            Self::powerpc_unknown_linux_gnu => "powerpc-unknown-linux-gnu",
            Self::mips_unknown_linux_gnu => "mips-unknown-linux-gnu",
            Self::arm_linux_androideabi => "arm-linux-androideabi",
        }
    }
}
