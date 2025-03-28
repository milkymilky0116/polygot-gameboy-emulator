package gameboy

type Register struct {
	A uint8
	B uint8
	C uint8
	D uint8
	E uint8
	H uint8
	L uint8
	F uint8

	PC uint16
	SP uint16
}

type Flag uint8

const (
	Z Flag = 0x80
	N Flag = 0x40
	H Flag = 0x20
	C Flag = 0x10
)

func NewRegister() *Register {
	register := &Register{
		A:  0,
		B:  0,
		C:  0,
		D:  0,
		E:  0,
		H:  0,
		L:  0,
		F:  0,
		PC: 0,
		SP: 0,
	}
	register.reset()
	return register
}

func (r *Register) reset() {
	r.A = 0x11
	r.B = 0x00
	r.C = 0x13
	r.D = 0x00
	r.E = 0xD8
	r.H = 0x01
	r.L = 0x4D
	r.F = 0xB0
	r.PC = 0x0100
	r.SP = 0xFFFE
}

func (r *Register) GetAF() uint16 {
	return uint16(r.A)<<8 | uint16(r.F&0xF0)
}

func (r *Register) GetBC() uint16 {
	return uint16(r.B)<<8 | uint16(r.C)
}

func (r *Register) GetDE() uint16 {
	return uint16(r.D)<<8 | uint16(r.E)
}

func (r *Register) GetHL() uint16 {
	return uint16(r.H)<<8 | uint16(r.L)
}

func (r *Register) SetAF(value uint16) {
	r.A = uint8(value >> 8)
	r.F = uint8(value & 0x00F0)
}

func (r *Register) SetBC(value uint16) {
	r.B = uint8(value >> 8)
	r.C = uint8(value & 0x00FF)
}

func (r *Register) SetDE(value uint16) {
	r.D = uint8(value >> 8)
	r.E = uint8(value & 0x00FF)
}

func (r *Register) SetHL(value uint16) {
	r.H = uint8(value >> 8)
	r.L = uint8(value & 0x00FF)
}

func (r *Register) SetF(flag uint8) {
	r.F = flag & 0xF0
}

func (r *Register) GetFlag(flag Flag) bool {
	mask := uint8(flag)
	return r.F&mask > 0
}

func (r *Register) SetFlag(flag Flag, value bool) {
	mask := uint8(flag)
	if value {
		r.F |= mask
	} else {
		r.F &= ^mask
	}
	r.F &= 0xF0
}

func (r *Register) IncrementHL() uint16 {
	temp := r.GetHL()
	r.SetHL(temp + 1)
	return temp
}

func (r *Register) DecrementHL() uint16 {
	temp := r.GetHL()
	r.SetHL(temp - 1)
	return temp
}
