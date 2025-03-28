use rust_emulator::gameboy::register::{self, Flag, Register};

#[tokio::test]
async fn test_register() {
    let mut register = register::Register::new();
    register.set_a(0x12);
    register.set_b(0x34);
    register.set_c(0x45);
    register.set_d(0x56);
    register.set_e(0x67);
    register.set_h(0x78);
    register.set_l(0x89);

    register.set_f(0x23);

    assert_eq!(register.get_af(), 0x1220);
    assert_eq!(register.get_bc(), 0x3445);
    assert_eq!(register.get_de(), 0x5667);
    assert_eq!(register.get_hl(), 0x7889);

    register.set_af(0x1111);
    assert_eq!(register.get_af(), 0x1110);

    register.set_bc(0x1111);
    assert_eq!(register.get_bc(), 0x1111);

    register.set_de(0x1111);
    assert_eq!(register.get_de(), 0x1111);

    register.set_hl(0x1111);
    assert_eq!(register.get_hl(), 0x1111);
}

#[tokio::test]
async fn test_flag() {
    let mut register = register::Register::new();
    assert_eq!(register.get_f() & 0x0F, 0);
    register.set_f(0x00);
    let flags = vec![Flag::C, Flag::H, Flag::N, Flag::Z];
    for flag in flags {
        assert_eq!(register.get_flag(flag), false);

        register.set_flag(flag, true);
        assert_eq!(register.get_flag(flag), true);

        register.set_flag(flag, false);
        assert_eq!(register.get_flag(flag), false);
    }
}

#[tokio::test]
async fn test_hl() {
    let mut register = Register::new();
    register.set_hl(0x1234);

    assert_eq!(register.get_hl(), 0x1234);
    assert_eq!(register.dec_hl(), 0x1234);
    assert_eq!(register.dec_hl(), 0x1233);
    assert_eq!(register.dec_hl(), 0x1232);
    assert_eq!(register.inc_hl(), 0x1231);
    assert_eq!(register.inc_hl(), 0x1232);
    assert_eq!(register.inc_hl(), 0x1233);

    assert_eq!(register.get_hl(), 0x1234);
}
