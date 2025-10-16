#!/bin/bash

if [[ -z "$MYSQL_USER" ]] || [[ -z "$MYSQL_PASSWD" ]]; then
		echo "(DB_ERROR):: one of this variable is missing MYSQL_USER or MYSQL_PASSWD please set it."
		exit 1
fi

echo "SETTING UP DATABASE WITH SAMPLE DATA..."

MYSQL_FILE="$HOME/Nebulon/projects/fm_server/setup_db.sql"
sudo mariadb -u $MYSQL_USER -p$MYSQL_PASSWD < $MYSQL_FILE

if [ $? -eq 0 ]; then
		echo "DONE"
else
		echo "(DB_ERROR):: Failed to setup the DB"
fi
