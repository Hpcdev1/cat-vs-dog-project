pub fn vote_page() -> String {
    r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <title>Cat vs Dog — Vote</title>
    <style>
        * { margin:0; padding:0; box-sizing:border-box; }
        body {
            font-family: 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #1a1a2e, #16213e);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .container { text-align:center; padding:40px; }
        h1 {
            font-size:3rem;
            margin-bottom:10px;
            background: linear-gradient(to right, #f093fb, #f5576c);
            -webkit-background-clip:text;
            -webkit-text-fill-color:transparent;
        }
        p.subtitle { color:#aaa; margin-bottom:50px; font-size:1.1rem; }
        .cards { display:flex; gap:40px; justify-content:center; flex-wrap:wrap; }
        .card {
            background:rgba(255,255,255,0.05);
            border:2px solid rgba(255,255,255,0.1);
            border-radius:20px;
            padding:50px 60px;
            cursor:pointer;
            transition:all 0.3s ease;
            width:220px;
        }
        .card:hover { transform:translateY(-10px); }
        .card.cat:hover { border-color:#f093fb; box-shadow:0 20px 40px rgba(240,147,251,0.3); }
        .card.dog:hover { border-color:#4facfe; box-shadow:0 20px 40px rgba(79,172,254,0.3); }
        .emoji { font-size:5rem; display:block; margin-bottom:20px; }
        .card h2 { font-size:1.8rem; margin-bottom:10px; }
        .btn {
            margin-top:25px; padding:12px 30px;
            border:none; border-radius:50px;
            font-size:1rem; font-weight:bold;
            cursor:pointer; width:100%;
        }
        .btn-cat { background:linear-gradient(to right,#f093fb,#f5576c); color:white; }
        .btn-dog { background:linear-gradient(to right,#4facfe,#00f2fe); color:white; }
        #message {
            margin-top:30px; padding:15px 30px;
            border-radius:10px; font-size:1.1rem; display:none;
        }
        .success { background:rgba(0,200,100,0.2); border:1px solid #00c864; color:#00c864; }
        .error { background:rgba(255,50,50,0.2); border:1px solid #ff3232; color:#ff3232; }
        .results-link {
            margin-top:40px; display:inline-block; color:#aaa;
            text-decoration:none; border:1px solid rgba(255,255,255,0.2);
            padding:10px 25px; border-radius:50px;
        }
        .results-link:hover { color:white; border-color:white; }
    </style>
</head>
<body>
    <div class="container">
        <h1> Cat vs Dog</h1>
        <p class="subtitle">Votez pour votre animal préféré !</p>
        <div class="cards">
            <div class="card cat" onclick="voter('cat')">
                <span class="emoji">🐱</span>
                <h2>Cat</h2>
                <button class="btn btn-cat">Voter pour Cat</button>
            </div>
            <div class="card dog" onclick="voter('dog')">
                <span class="emoji">🐶</span>
                <h2>Dog</h2>
                <button class="btn btn-dog">Voter pour Dog</button>
            </div>
        </div>
        <div id="message"></div>
        <a href="http://localhost:8081" class="results-link">
             Voir les résultats →
        </a>
    </div>
    <script>
        async function voter(choix) {
            const msg = document.getElementById('message');
            try {
                const r = await fetch('/vote', {
                    method:'POST',
                    headers:{'Content-Type':'application/json'},
                    body:JSON.stringify({choice:choix})
                });
                const data = await r.json();
                msg.style.display = 'block';
                if (r.ok) {
                    msg.className = 'success';
                    msg.textContent = '✅ Vote enregistré pour ' + (choix==='cat'?'🐱 Cat':'🐶 Dog') + ' !';
                } else {
                    msg.className = 'error';
                    msg.textContent = '❌ ' + data.error;
                }
            } catch(e) {
                msg.style.display = 'block';
                msg.className = 'error';
                msg.textContent = '❌ Impossible de contacter le serveur';
            }
            setTimeout(()=>{ msg.style.display='none'; }, 3000);
        }
    </script>
</body>
</html>"#.to_string()
}
