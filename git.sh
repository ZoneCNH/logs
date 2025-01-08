#!/bin/bash

# 初始化 git 仓库
git init

# 添加远程仓库



# 添加所有文件
git add .

commit_hash=$(git rev-parse HEAD)

tag_name="v1.0.11"

git tag -a $tag_name -m "自动打标签: $commit_hash"

# 提交更改
git commit -m "feat: add date variable support for log file naming

- Add support for date variables: ${YEAR}, ${MONTH}, ${DAY}
- Update log file naming format to include date
- Add time_support feature for date functionality
- Update documentation with new features"

git remote add origin git@github.com:ZoneCNH/logs.git

# 切换到 main 分支
git branch -M main

# 推送到远程仓库
git push -u origin main

# 创建新标签

# 将新标签推送到远程仓库
git push origin $tag_name