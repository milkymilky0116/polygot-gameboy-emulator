package gameboy

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRegister(t *testing.T) {
	t.Run("Testing Register", func(t *testing.T) {
		register := NewRegister()
		register.A = 0x12
		register.B = 0x34
		register.C = 0x45
		register.D = 0x56
		register.E = 0x67
		register.H = 0x78
		register.L = 0x89
		register.SetF(0x23)

		assert.Equal(t, uint16(0x1220), register.GetAF())
		assert.Equal(t, uint16(0x3445), register.GetBC())
		assert.Equal(t, uint16(0x5667), register.GetDE())
		assert.Equal(t, uint16(0x7889), register.GetHL())

		register.SetAF(0x1111)
		assert.Equal(t, uint16(0x1110), register.GetAF())

		register.SetBC(0x1111)
		assert.Equal(t, uint16(0x1111), register.GetBC())

		register.SetDE(0x1111)
		assert.Equal(t, uint16(0x1111), register.GetDE())

		register.SetHL(0x1111)
		assert.Equal(t, uint16(0x1111), register.GetHL())
	})

	t.Run("Testing Flag", func(t *testing.T) {
		register := NewRegister()
		register.SetF(0x00)
		flags := []Flag{
			C, H, N, Z,
		}
		for _, flag := range flags {
			assert.Equal(t, register.GetFlag(flag), false)

			register.SetFlag(flag, true)
			assert.Equal(t, register.GetFlag(flag), true)

			register.SetFlag(flag, false)
			assert.Equal(t, register.GetFlag(flag), false)
		}
	})

	t.Run("Test HL", func(t *testing.T) {
		register := NewRegister()
		register.SetHL(0x1234)
		assert.Equal(t, uint16(0x1234), register.GetHL())
		assert.Equal(t, uint16(0x1234), register.DecrementHL())
		assert.Equal(t, uint16(0x1233), register.DecrementHL())
		assert.Equal(t, uint16(0x1232), register.DecrementHL())
		assert.Equal(t, uint16(0x1231), register.IncrementHL())
		assert.Equal(t, uint16(0x1232), register.IncrementHL())
		assert.Equal(t, uint16(0x1233), register.IncrementHL())
		assert.Equal(t, uint16(0x1234), register.GetHL())
	})
}
