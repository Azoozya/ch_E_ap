<?php
$servername = "localhost";
$database = "db_nico";
$username = "front";
$password = "front";

try {
    $db = new PDO("mysql:host=$servername;dbname=$database", $username, $password);
    $db->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);

    $data = json_decode(file_get_contents('php://input'), true);
    $return = [];
    
    if (($data['action'] === 'check-step-1') || ($data['action'] === 'check-step-2')) {
        $query = $db->prepare("SELECT id FROM users WHERE name = ?;");
        $query->execute([$data['user']]);
        $result = $query->fetchAll();

        if (count($result) != 0) {
            $user_id = $result[0]['id'];

            if ($data['action'] === 'check-step-1') {
                $random = generateRandom32();
                $query = $db->prepare("UPDATE challenges SET nonce = ?, expiration = NOW() + INTERVAL 5 MINUTE WHERE user = ?;");
                $query->execute([$random, $user_id]);
                
                $return = new stdClass();
                $return->secret = $random;
            } else if ($data['action'] === 'check-step-2') {
                $query = $db->prepare("SELECT nonce FROM challenges WHERE user = ? AND expiration >= NOW();");
                $query->execute([$user_id]);

                $result = $query->fetchAll();
                $return = new stdClass();
                if ((count($result) != 0) && (verifySignature($result[0]['nonce'], $data['signed']))) {
                    $return->response = true;
                } else {
                    $return->response = false;
                }
            }
        }
    }

    $json = json_encode($return);
    print_r($json);
} catch(PDOException $e) {
    echo "Connection failed: " . $e->getMessage();
}

function generateRandom32() {
    $reference = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
    $random = '';
    for ($i = 0; $i < 6; $i++) {
        $random .= $reference[rand(0, strlen($reference) - 1)];
    }
    return $random;
}
function verifySignature($nonce, $signed) {
    return ($signed == '1235446');
}
?>