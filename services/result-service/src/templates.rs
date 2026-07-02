pub fn results_page() -> String {
    r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <title>Cat vs Dog — Résultats</title>
    <style>
        * { margin:0; padding:0; box-sizing:border-box; }
        body {
            font-family:'Segoe UI',sans-serif;
            background:linear-gradient(135deg,#1a1a2e,#16213e);
            min-height:100vh; display:flex;
            align-items:center; justify-content:center; color:white;
        }
        .container { text-align:center; padding:40px; width:100%; max-width:700px; }
        h1 {
            font-size:2.8rem; margin-bottom:10px;
            background:linear-gradient(to right,#f093fb,#4facfe);
            -webkit-background-clip:text; -webkit-text-fill-color:transparent;
        }
        .live {
            display:inline-block; background:#ff3232; color:white;
            font-size:0.75rem; padding:3px 10px; border-radius:20px;
            margin-bottom:40px; animation:pulse 1.5s infinite;
        }
        @keyframes pulse { 0%,100%{opacity:1} 50%{opacity:0.5} }
        .scores { display:flex; gap:30px; justify-content:center; margin-bottom:40px; flex-wrap:wrap; }
        .score-card {
            background:rgba(255,255,255,0.05); border-radius:20px;
            padding:35px 50px; border:2px solid rgba(255,255,255,0.1); min-width:200px;
        }
        .score-card.cat { border-color:rgba(240,147,251,0.4); }
        .score-card.dog { border-color:rgba(79,172,254,0.4); }
        .emoji { font-size:4rem; display:block; margin-bottom:15px; }
        .score-card h2 { font-size:1.4rem; color:#aaa; margin-bottom:10px; }
        .number { font-size:4rem; font-weight:bold; }
        .cat .number { color:#f093fb; }
        .dog .number { color:#4facfe; }
        .total { font-size:1.1rem; color:#aaa; margin-bottom:30px; }
        .barre-container {
            background:rgba(255,255,255,0.1); border-radius:50px;
            height:20px; overflow:hidden; margin-bottom:10px; display:flex;
        }
        .barre-cat { background:linear-gradient(to right,#f093fb,#f5576c); height:100%; transition:width 0.5s ease; }
        .barre-dog { background:linear-gradient(to right,#4facfe,#00f2fe); height:100%; transition:width 0.5s ease; }
        .pourcentages { display:flex; justify-content:space-between; color:#aaa; font-size:0.9rem; margin-bottom:40px; }
        .vote-link {
            display:inline-block; color:#aaa; text-decoration:none;
            border:1px solid rgba(255,255,255,0.2); padding:10px 25px; border-radius:50px;
        }
        .vote-link:hover { color:white; border-color:white; }
        #status { font-size:0.8rem; color:#555; margin-top:20px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Résultatsen live</h1>
        <div class="scores">
            <div class="score-card cat">
                <span class="emoji">🐱</span>
                <h2>Cat</h2>
                <span class="number" id="cats">0</span>
            </div>
            <div class="score-card dog">
                <span class="emoji">🐶</span>
                <h2>Dog</h2>
                <span class="number" id="dogs">0</span>
            </div>
        </div>
        <p class="total">Total : <strong id="total">0</strong> votes</p>

        <a href="http://localhost:8080" class="vote-link"> Voter →</a>
    </div>
    <script>
        function update(data) {
            document.getElementById('cats').textContent = data.cats;
            document.getElementById('dogs').textContent = data.dogs;
            document.getElementById('total').textContent = data.total;
            const t = data.total || 1;
            const pc = Math.round((data.cats/t)*100);
            const pd = 100-pc;
            document.getElementById('barre-cat').style.width = pc+'%';
            document.getElementById('barre-dog').style.width = pd+'%';
            document.getElementById('pct-cat').textContent = '🐱 '+pc+'%';
            document.getElementById('pct-dog').textContent = pd+'% 🐶';
        }
        const es = new EventSource('/results/events');
        es.addEventListener('score', e => {
            update(JSON.parse(e.data));
            document.getElementById('status').textContent = '🟢 Live — ' + new Date().toLocaleTimeString();
        });
        es.onerror = () => {
            document.getElementById('status').textContent = '🔴 Reconnexion...';
        };
        fetch('/results').then(r=>r.json()).then(update);
    </script>
</body>
</html>"#.to_string()
}
