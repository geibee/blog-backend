# Run
1. Prepare sqlite file

```
$ sqlite3 db/blog.sqlite

> CREATE TABLE posts (id integer primary key autoincrement, caption text, image_url text, created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),  updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')));
```

2. Set environment variables for AWS

```
# [prerequisite] set environment variables on your shell.
$ printenv | grep AWS > .env
$ echo "IMAGE_BUCKET=<your-image-bucket-name>" >> .env
```

3. Run application

```
$ docker run -d -p 3000:3000 --env-file $(pwd)/.env --volume $(pwd)/db:/mnt/efs/db <image-id>
```