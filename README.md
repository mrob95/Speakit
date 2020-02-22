# Speakit

## Installation
```
pip install speakit
```

## Documentation
The module exports two functions:

```
from speakit import split_symbol, split_symbols

def split_symbol(symbol: str, numbers: bool = True, max_len: int = 0) -> str
def split_symbol(symbols: List[str], numbers: bool = True, max_len: int = 0) -> List[str]
```

## Examples
```
split_symbol("999") -> "nine nine nine"
split_symbol("99module_aTestCase") -> "nine nine module a Test Case"
split_symbol("0_A1B2C3DEF99") -> "zero A one B two C three D E F nine nine"
split_symbol("__init__") -> "init"
split_symbol("99module_aTestCase", numbers=False, max_len=3) -> "module a Test"
```