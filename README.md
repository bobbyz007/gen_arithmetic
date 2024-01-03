# gen_arithmetic
命令示例用法如下。 更高级的选项请参考对应命令的 -h 帮助选项
## add/minus
```shell
# 获取帮助
.\gen_arithmatic.exe add-minus -h

# 生成200个题目， 生成加法(-c +指定)，数据范围是[0,99]
gen_arithmatic add-minus -n 200 -c + -r 99

# 生成200个题目， 生成整十的加法(-c +0指定)，数据范围是[0,99]
gen_arithmatic add-minus -n 200 -c +0 -r 99

# 生成200个题目， 生成整十的加法(-c +0指定)，数据范围是[80,99]
gen_arithmatic add-minus -n 200 -c +0 -l 80 -r 99

# 生成200个题目， 生成整十的减法(-c -0指定)，数据范围是[80,99]
gen_arithmatic add-minus -n 200 -c -0 -l 80 -r 99

# 生成200个题目， 生成减法(-c -指定)，数据范围是[80,99]
gen_arithmatic add-minus -n 200 -c - -l 80 -r 99

# 生成200个题目， 生成加减法随机混合(-c x指定)，数据范围是[80,99]
gen_arithmatic add-minus -n 200 -c x -l 80 -r 99

# 生成200个题目， 生成整十的加减法随机混合(-c x0指定)，数据范围是[80,99]
gen_arithmatic add-minus -n 200 -c +0 -l 80 -r 99
```

## missing number
```shell
# 获取帮助
.\gen_arithmatic.exe missing-number -h

# 生成100个题目，gap数为3，每个gap包含2个数字，数字范围(-l, -r指定）是[0,999]，累加值step(-s)是3
gen_arithmatic missing-number -n 100 -m 2 -g 3 -r 999 -s 3

# 生成100个题目，gap数为3，每个gap包含2个数字，数字范围(-l, -r指定）是[200,999]，累加值step(-s)是3
gen_arithmatic missing-number -n 100 -m 2 -g 3 -l 200 -r 999 -s 3

# step可以指定负数，并且-t表示序列的起始数字是step的倍数
run --package gen_arithmatic --bin gen_arithmatic -- missing-number -n 100 -m 2 -g 3 -r 120 -s -10 -t
```
