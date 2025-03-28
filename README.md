# Gameboy Emulator in Rust, Go

본 레포지토리는 게임보이 에뮬레이터를 Rust와 Go로 구현하면서 공부한 기록들을 남겼습니다.

## 1. Basic Architecture

### 게임보이 주요 하드웨어 구성 요소

1. 8bit CPU (Sharp LR35902)
2. 8KB Internal RAM
3. 8KB VRAM
4. Timer, Interrupt Controller
5. Cartridge MBC (Memory Bank Controller)

### 개발 계획

1. CPU 에뮬레이션
2. Memory 관리
3. 그래픽 렌더링
