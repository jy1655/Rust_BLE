# Rust BLE Peripheral

임베디드 Linux 환경에서 BLE(Bluetooth Low Energy) Peripheral을 구현하는 Rust 프로젝트입니다.

## 개요

이 프로젝트는 Rust와 BlueR 라이브러리를 사용하여 BLE Peripheral 장치를 구현합니다. GATT 서비스와 특성(characteristic)을 정의하고, BLE Central 장치(스마트폰, 컴퓨터 등)와 통신할 수 있습니다.

## 주요 기능

- ✅ BLE Peripheral 모드로 동작
- ✅ 커스텀 GATT 서비스 및 특성 정의
- ✅ Read/Write 오퍼레이션 지원
- ✅ BLE Advertisement 기능
- ✅ 비동기 이벤트 처리
- ✅ 로깅 및 디버그 기능

## 시스템 요구사항

### 하드웨어
- Bluetooth Low Energy를 지원하는 블루투스 어댑터
- Linux 운영체제 (Raspberry Pi, 임베디드 Linux 보드 등)

### 소프트웨어
- Rust 1.70 이상
- BlueZ 5.60 이상
- D-Bus 시스템
- 루트 권한 또는 적절한 udev 규칙

### 필요한 시스템 패키지 (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y \
    bluetooth \
    bluez \
    libbluetooth-dev \
    libdbus-1-dev \
    pkg-config \
    build-essential
```

## 설치 및 빌드

```bash
# 저장소 클론
git clone <repository-url>
cd rust_ble_peripheral

# 의존성 설치 및 빌드
cargo build --release
```

## 사용 방법

### 1. 블루투스 서비스 확인

```bash
# 블루투스 서비스 상태 확인
sudo systemctl status bluetooth

# 서비스가 중지된 경우 시작
sudo systemctl start bluetooth
sudo systemctl enable bluetooth
```

### 2. 프로그램 실행

```bash
# 개발 모드에서 실행
RUST_LOG=info cargo run

# 또는 빌드된 바이너리 실행
sudo ./target/release/rust_ble_peripheral
```

### 3. 연결 테스트

스마트폰이나 다른 BLE Central 장치에서 "RustBLE" 장치를 스캔하고 연결할 수 있습니다.

## 프로젝트 구조

```
src/
├── main.rs                 # 메인 애플리케이션 코드
├── Cargo.toml             # 프로젝트 설정 및 의존성
└── README.md              # 프로젝트 문서
```

## 코드 구조 설명

### 주요 컴포넌트

1. **GATT Service**: 커스텀 서비스 UUID로 BLE 서비스 정의
2. **Characteristic**: Read/Write 가능한 특성 정의
3. **Advertisement**: BLE 장치 검색을 위한 광고 설정
4. **Event Loop**: 비동기 이벤트 처리 및 사용자 입력 처리

### UUID 설정

```rust
// 커스텀 서비스 UUID (필요에 따라 변경 가능)
const SERVICE_UUID: Uuid = Uuid::from_u128(0x12345678_1234_1234_1234_123456789abc);
const CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x87654321_4321_4321_4321_cba987654321);
```

## BLE 통신 예제

### Read 요청 처리
Central 장치에서 특성 값을 읽을 때 "Hello from Rust BLE Peripheral!" 메시지를 반환합니다.

### Write 요청 처리
Central 장치에서 데이터를 쓰면 콘솔에 로그로 출력됩니다.

## 문제 해결

### 권한 오류
```bash
# 사용자를 bluetooth 그룹에 추가
sudo usermod -a -G bluetooth $USER

# 또는 sudo로 실행
sudo cargo run
```

### BlueZ 서비스 오류
```bash
# BlueZ 서비스 재시작
sudo systemctl restart bluetooth

# 어댑터 상태 확인
bluetoothctl show
```

### D-Bus 연결 오류
```bash
# D-Bus 서비스 확인
sudo systemctl status dbus

# 필요시 재시작
sudo systemctl restart dbus
```

## 커스터마이징

### 1. 서비스/특성 UUID 변경
`main.rs`의 `SERVICE_UUID`와 `CHARACTERISTIC_UUID` 상수를 수정하세요.

### 2. Advertisement 이름 변경
`setup_advertisement` 함수에서 `local_name`을 변경하세요.

### 3. 특성 데이터 변경
`DeviceCharacteristic::new()`에서 초기 값을 수정하거나 동적 데이터를 추가하세요.

## 개발 환경

### 로그 레벨 설정
```bash
# 상세 로그
RUST_LOG=debug cargo run

# 정보 로그만
RUST_LOG=info cargo run
```

### 디버그 빌드
```bash
cargo build
```

### 릴리즈 빌드
```bash
cargo build --release
```

## 참고 자료

- [BlueR 문서](https://docs.rs/bluer)
- [BlueZ 공식 문서](http://www.bluez.org/documentation/)
- [Bluetooth Low Energy 스펙](https://www.bluetooth.com/specifications/bluetooth-core-specification/)

## 라이선스

MIT License

## 기여

이슈나 풀 리퀘스트를 통해 프로젝트에 기여해 주세요.