# GdSerial - Biblioteca de Comunicação Serial para Godot 4

[![GitHub Downloads](https://img.shields.io/github/downloads/SujithChristopher/gdserial/total?style=flat-square&label=Downloads&color=blue)](https://github.com/SujithChristopher/gdserial/releases)
[![Latest Release](https://img.shields.io/github/v/release/SujithChristopher/gdserial?style=flat-square&label=Latest)](https://github.com/SujithChristopher/gdserial/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=flat-square)](https://github.com/SujithChristopher/gdserial/blob/main/LICENSE)
[![Godot 4](https://img.shields.io/badge/Godot-4.x-478CBF?style=flat-square&logo=godot-engine&logoColor=white)](https://godotengine.org)
[![Rust 2021](https://img.shields.io/badge/Rust-2021_edition-orange?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)

**Idioma / Language**: [English](README.md) | [简体中文](README.zh-CN.md) | Português do Brasil

<img src="addons/gdserial/icon.png" alt="Ícone GdSerial" width="64" height="64" align="left" style="margin-right: 20px;">

Uma biblioteca de comunicação serial de alto desempenho baseada em Rust para o motor de jogo Godot 4, fornecendo funcionalidades semelhantes ao PySerial através do gdext. Habilite comunicação direta com hardware Arduino, ESP32, sensores, modems e dispositivos IoT em seus jogos e aplicações Godot.

<br clear="left">

## Recursos

- **Integração Arduino & ESP32**: Comunicação direta com microcontroladores e placas de desenvolvimento
- **Suporte a Sensores de Hardware**: Interface com sensores de temperatura, movimento, GPS e ambientais
- **Comunicação Serial Multiplataforma**: Funciona no Windows, Linux e macOS
- **API Semelhante ao PySerial**: Interface familiar para desenvolvedores Python migrando para Godot
- **Integração com Motor de Jogo**: Tipos nativos do Godot 4 e tratamento de erros para fluxo de trabalho sem fricção
- **Comunicação com Dispositivos IoT**: Conecte-se com módulos WiFi, adaptadores Bluetooth e modems celulares
- **Streaming de Dados em Tempo Real**: Operações binárias e de texto com baixa latência para aplicações responsivas
- **Descoberta Automática de Portas**: Enumeração e identificação automática de dispositivos seriais disponíveis

## Suporte de Plataformas

| Plataforma | Arquitetura | Status | Exemplos de Porta |
| :--- | :--- | :--- | :--- |
| **Windows** | x64 | ✅ Suportado | `COM1`, `COM3`, `COM8` |
| **Windows** | ARM64 | ✅ Suportado | `COM1`, `COM3`, `COM8` |
| **Linux** | x64 | ✅ Suportado | `/dev/ttyUSB0`, `/dev/ttyACM0` |
| **Linux** | ARM64 | ✅ Suportado | `/dev/ttyUSB0`, `/dev/ttyACM0` |
| **macOS** | x64 (Intel) | ✅ Suportado | `/dev/tty.usbserial-*`, `/dev/tty.usbmodem*` |
| **macOS** | ARM64 (Apple Silicon) | ✅ Suportado | `/dev/tty.usbserial-*`, `/dev/tty.usbmodem*` |

> **Usuários macOS**: Se encontrar avisos de "malware" ou erros de carregamento de biblioteca, consulte [MACOS_SECURITY.md](MACOS_SECURITY.md) para soluções.

## Instalação

### Opção 1: GitHub Releases (Recomendado)

1. Baixe a versão mais recente para sua plataforma em [GitHub Releases](https://github.com/SujithChristopher/gdserial/releases)
2. Extraia a pasta `addons/gdserial` para o diretório `addons/` do seu projeto
3. Vá em Projeto > Configurações do Projeto > Plugins
4. Habilite o plugin "GdSerial - Serial Communication Library"

### Opção 2: Compilar a partir do Código-Fonte

#### Pré-requisitos

- Rust (versão estável mais recente)
- Godot 4.2+
- Git

#### Etapas de Compilação

1. Clone este repositório:

```bash
git clone https://github.com/SujithChristopher/gdserial.git
cd gdserial
```

2. Compile a biblioteca:

```bash
# Linux/Mac
./build_release.sh

# Windows
build_release.bat
```

3. O plugin estará pronto na pasta `addons/gdserial` com as bibliotecas compiladas.

## Documentação no Editor

GdSerial inclui documentação de referência de classes que aparece diretamente na Ajuda integrada do Godot (F1). O Godot seleciona o idioma automaticamente com base no idioma do seu sistema.

| Idioma | Pasta |
| :--- | :--- |
| English | [doc/en/](addons/gdserial/doc/en/) |
| 简体中文 | [doc/zh_CN/](addons/gdserial/doc/zh_CN/) |
| Português do Brasil | [doc/pt_BR/](addons/gdserial/doc/pt_BR/) |

## Referência da API

### Classes Principais

#### `GdSerial` (API Básica)

API simples semelhante ao PySerial para controle síncrono direto de uma única porta.

##### Gerenciamento de Porta

- `list_ports() -> Dictionary` — Obter todas as portas seriais disponíveis (índice -> dicionário de informações)
- `set_port(port_name: String)` — Definir a porta a usar (ex.: "COM3", "/dev/ttyUSB0")
- `set_baud_rate(rate: int)` — Definir taxa de baud (padrão: 9600)
- `set_data_bits(bits: int)` — Definir bits de dados (6, 7, 8)
- `set_parity(type: int)` — Definir paridade (0: Nenhuma, 1: Ímpar, 2: Par)
- `set_stop_bits(bits: int)` — Definir bits de parada (1, 2)
- `set_flow_control(type: int)` — Definir controle de fluxo (0: Nenhum, 1: Software, 2: Hardware)
- `set_timeout(timeout_ms: int)` — Definir timeout de leitura em milissegundos (padrão: 1000)
- `open() -> bool` — Abrir a porta serial
- `close()` — Fechar a porta serial
- `is_open() -> bool` — Verificar se a porta está aberta (realiza teste ativo de conexão)

##### Operações de Dados

- `write(data: PackedByteArray) -> bool` — Escrever bytes brutos
- `write_string(data: String) -> bool` — Escrever dados de string
- `writeline(data: String) -> bool` — Escrever string com nova linha
- `read(size: int) -> PackedByteArray` — Ler bytes brutos
- `read_string(size: int) -> String` — Ler e converter para string
- `readline() -> String` — Ler até o caractere de nova linha

##### Utilitários

- `bytes_available() -> int` — Obter número de bytes aguardando leitura
- `clear_buffer() -> bool` — Limpar buffers de entrada/saída

#### `GdSerialManager` (API Avançada)

Gerenciador assíncrono multi-porta usando threads em segundo plano e sinais. Ideal para aplicações complexas.

##### Métodos

- `list_ports() -> Dictionary` — Igual a `GdSerial.list_ports()`
- `open(name: String, baud: int, timeout: int) -> bool` — Abrir porta em modo RAW
- `open_buffered(name: String, baud: int, timeout: int, mode: int) -> bool` — Abrir porta com modo de buffer (0: raw, 1: linha, 2: delimitador personalizado)
- `close(name: String)` — Fechar e parar thread de leitura; emite `port_disconnected`
- `is_open(name: String) -> bool` — Verificar se uma porta específica está aberta
- `write(name: String, data: PackedByteArray) -> bool` — Escrever bytes brutos em porta específica
- `reconfigure_port(...) -> bool` — Atualizar configurações em uma porta aberta
- `set_delimiter(name: String, delimiter: int) -> bool` — Definir byte delimitador personalizado para modo 2
- `poll_events() -> Array` — **Crucial**: Chame em `_process` para emitir sinais e obter eventos

##### Sinais

- `data_received(port: String, data: PackedByteArray)` — Emitido quando novos dados chegam
- `port_disconnected(port: String)` — Emitido quando uma porta é perdida/desconectada (fechamento manual ou hardware)

### Opção A: Gerenciador Assíncrono (Recomendado para UI não bloqueante)

```gdscript
extends Node

var manager: GdSerialManager

func _ready():
    manager = GdSerialManager.new()
    manager.data_received.connect(_on_data)
    manager.port_disconnected.connect(_on_disconnect)

    # Modo 0: RAW (emite todos os chunks), 1: LINE_BUFFERED (aguarda \n), 2: CUSTOM_DELIMITER
    if manager.open("COM3", 9600, 1000):
        print("Conectado à COM3")

func _process(_delta):
    # Isto aciona os sinais acima
    manager.poll_events()

func _on_data(port: String, data: PackedByteArray):
    print("Dados de ", port, ": ", data.get_string_from_utf8())

func _on_disconnect(port: String):
    print("Conexão perdida com ", port)
```

### Opção B: API Bloqueante Simples

```gdscript
extends Node

var serial: GdSerial

func _ready():
    serial = GdSerial.new()
    
    # Listar portas disponíveis
    var ports: Dictionary = serial.list_ports()
    for i in ports:
        var info = ports[i]
        print("- ", info["port_name"], " (", info["device_name"], ")")
    
    serial.set_port("COM3")
    serial.set_baud_rate(115200)
    
    if serial.open():
        serial.writeline("Hello!")
        await get_tree().create_timer(0.1).timeout
        if serial.bytes_available() > 0:
            print("Resposta: ", serial.readline())
        serial.close()
```

> **Nota**: A classe GdSerial fica disponível automaticamente assim que o plugin é habilitado. Sem imports necessários!

## Casos de Uso Comuns

### Comunicação com Arduino

```gdscript
# Síncrono (simples, bloqueante)
serial.writeline("GET_SENSOR")
var reading = serial.readline()
print("Valor do sensor: ", reading)
```

Para comunicação assíncrona multi-porta, use `GdSerialManager` com modo 1 (linha):
```gdscript
manager.open_buffered("COM3", 9600, 1000, 1)  # Modo 1: aguarda nova linha
```

### Comandos AT (Modems, módulos WiFi)

```gdscript
serial.writeline("AT+VERSION?")
var version = serial.readline()
print("Versão do módulo: ", version)
```

### Transferência de Dados Binários

```gdscript
# Abordagem síncrona
var data = PackedByteArray([0x01, 0x02, 0x03, 0x04])
serial.write(data)
var response = serial.read(10)
```

Para protocolos binários assíncronos, use `GdSerialManager` com modo 0 (raw) ou modo 2 (delimitador personalizado):
```gdscript
manager.open_buffered("COM3", 9600, 1000, 0)  # Modo 0: emite todos os chunks imediatamente
# ou
manager.open_buffered("COM3", 9600, 1000, 2)  # Modo 2: aguarda byte delimitador
manager.set_delimiter("COM3", 0xFF)            # Definir marcador de fim personalizado
```

## Notas Específicas por Plataforma

### Windows

- Nomes de porta são tipicamente `COM1`, `COM2`, etc.
- Privilégios de administrador podem ser necessários para alguns dispositivos

### Linux

- Nomes de porta são tipicamente `/dev/ttyUSB0`, `/dev/ttyACM0`, etc.
- O usuário deve estar no grupo `dialout` para acessar portas seriais:

  ```bash
  sudo usermod -a -G dialout $USER
  ```

### macOS

- Nomes de porta são tipicamente `/dev/tty.usbserial-*` ou `/dev/tty.usbmodem*`

## Tratamento de Erros

A biblioteca fornece registro de erros abrangente através do sistema de logging integrado do Godot. Erros comuns incluem:

- Porta não encontrada ou inacessível
- Permissão negada (verifique permissões do usuário)
- Dispositivo desconectado durante operação
- Timeout durante operações de leitura

Verifique o console do Godot para mensagens de erro detalhadas quando operações falharem.

## Solução de Problemas

### Porta Não Encontrada

- Verifique se o dispositivo está conectado e reconhecido pelo SO
- Verifique o gerenciador de dispositivos (Windows) ou `dmesg` (Linux)
- Tente diferentes portas USB ou cabos

### Permissão Negada (Linux)

```bash
sudo usermod -a -G dialout $USER
# Saia e entre novamente para as mudanças terem efeito
```

### Problemas de Compilação

- Certifique-se de que o Rust está atualizado: `rustup update`
- Limpe o cache do cargo: `cargo clean`
- Verifique se todas as dependências estão disponíveis

## Estrutura do Plugin

```text
addons/gdserial/
├── plugin.cfg              # Configuração do plugin
├── plugin.gd               # Script de ativação do plugin
├── gdserial.gdextension    # Carregador de extensão
├── bin/                    # Bibliotecas compiladas
│   ├── windows-x86_64/
│   ├── linux-x86_64/
│   ├── linux-arm64/
│   ├── macos-x86_64/
│   └── macos-arm64/
├── doc/                    # Documentação no editor
│   ├── en/
│   ├── zh_CN/
│   └── pt_BR/
└── README.md
```

## Contribuindo

1. Faça um fork do repositório
2. Crie uma branch de funcionalidade
3. Faça suas alterações
4. Teste com os scripts de compilação
5. Envie um pull request

## Licença

Este projeto está licenciado sob a Licença MIT — consulte o arquivo LICENSE para detalhes.

## Dependências

- [gdext](https://github.com/godot-rust/gdext) — Bindings Rust para Godot 4
- [serialport](https://crates.io/crates/serialport) — Biblioteca serial multiplataforma
