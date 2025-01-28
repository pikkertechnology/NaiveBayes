#!/bin/sh
find /model/versions -type f -mtime +6 -exec rm -f {} \;