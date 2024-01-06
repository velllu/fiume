UPDATE users
SET session_id = $1
WHERE username = $2 AND password = $3;