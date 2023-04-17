#!/usr/bin/env bash
while true; do
    for md_path in */index.md; do
        html_path="${md_path%.md}.html"
        md_timestamp=$(date -r "$md_path" "+%s")
        html_timestamp=$(date -r "$html_path" "+%s" 2> /dev/null)
        if [ $? -ne 0 ] || (( $md_timestamp >= $html_timestamp )); then
            current_time=$(date "+%T")
            echo "[$current_time] Updating: $html_path"
            cd ./render-scripts/
            node render.js "../$md_path" > "../$html_path"
            cd ..
            current_time=$(date "+%T")
            echo "[$current_time]  Updated: $html_path"
        fi
    done
    sleep 1
done
