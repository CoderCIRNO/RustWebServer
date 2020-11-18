# Rust_web_server
基础部分是跟着《Rust权威指南》一步一步做的  
在此基础上又增加了路径判断功能和二进制文件传输功能，总的来就是解决了原书代码无法使用超链接，无法加载图片、音乐等问题  
简单的网页已经足以胜任，并发能力尚可（主要我这边测试能力有限）  
另外还解决了write()的一个panic，这个问题会消耗线程，最终让程序挂掉，解决之后稳定性还不错  

当前配置：线程池大小8，监听80端口，单次二进制传输65535Byte，index文件夹/var/www  
可在main.rs中自行修改  

本人纯Rust初学者，代码必然有大量不足，欢迎指点  

# 如何编译运行  
直接 cargo run 即可  

#更新历史  
2020-11-18 上传  
2020-11-18 修复了路径可以返回上层的安全漏洞（幸好我服务器没出事）  
2020-11-18 线程数增至8，日志输出优化  
2020-11-18 输出优化，handle_connection代码重构  
