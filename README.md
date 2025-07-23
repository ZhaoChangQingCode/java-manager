> ⚠️ 目前仅支持 MacOS
# java-manager
你还在频繁手动修改`JAVA_HOME`而烦恼吗？java-manager帮你抗下更换JVM的使命。只用一行代码，一键切换JVM，还能查看安装目录中所有的JVM发行版信息。

```shell
javamanager ls
```
输出：
```plain
 #     VER  VM       VENDOR               NAME
 0  23.0.1  GraalVM  Oracle Corporation   graalvm-jdk-23.0.1+11.1
 1  21.0.5  Openj9   IBM Corporation      jdk-21.0.5+11
 2* 23.0.2  Openj9   IBM Corporation      jdk-23.0.2+7 (Current)
 3  21.0.6  Openj9   IBM Corporation      jdk-21.0.6+7
 4  21.0.2  Hotspot  Oracle Corporation   jdk-21.0.2.jdk
 5  21.0.7  Zulu     Azul Systems, Inc.   zulu-21.jdk
```
如果我们要切换成序号为`0`的发行版 GraalVM，执行命令
```shell
sudo javamanager install -i 0
```
输出
```plain
Changing JVM
 #     VER  VM       VENDOR               NAME
 0* 23.0.1  GraalVM  Oracle Corporation   graalvm-jdk-23.0.1+11.1 (Current)
```
恭喜你，你已经切换JVM为 GraalVM了！

还能帮你一键安装JVM到系统目录
```shell
sudo javamanager install /path/to/jvm
```
这样会自动帮你把JVM安装到系统自带目录`/Library/Java/JavaVirtualMechines`目录，还能自动剔除掉 MacOS的`com.apple.quarantine`属性。

# 如何开始？
`sudo javamanager init`
或者手动向`~/.zprofile`文件写下：
```shell
export JAVA_HOME=/Library/Java/JavaVirtualMechines/Current
```
