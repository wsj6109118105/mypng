**本项目来自 [pngme_book](https://github.com/jrdngr/pngme_book)**
> 使用

cargo build/cargo build --release

之后生成对应的文件

> 命令行

```bash
>mypng --help
Usage: mypng <COMMAND>

Commands:
  encode  编码信息到图片
  decode  解码保存在块中的信息
  remove  移除保存编码信息的块
  print   打印图片字节信息
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
对应子命令行信息
```bash
>mypng encode --help
编码信息到图片

Usage: mypng encode <PATH> <MYTYPE> <MESSAGE>

Arguments:
  <PATH>     图片路径
  <MYTYPE>   请指定块类型，eg:ruSt，第一个字符必须为小写，第三个字符必须为大写
  <MESSAGE>  要编码的信息

Options:
  -h, --help  Print help
```

---
实现了在png文件中添加自己的块，移除指定类型的块，查看添加块中包含的信息。

如需了解更多细节请参考 [pngme_book](https://github.com/jrdngr/pngme_book)