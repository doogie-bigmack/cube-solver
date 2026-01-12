#!/bin/zsh
# ============================================================================
# AUTONOMOUS CLAUDE - 24 HOUR EXPERIMENT BOOTSTRAP
# Run this on the master node (claude) to initialize everything
# ============================================================================

set -e

echo "🤖 Autonomous Claude - 24 Hour Experiment Bootstrap"
echo "===================================================="
echo "Start Time: $(date)"
echo "Master Node: claude ($(hostname))"
echo "Worker Node: claudia"
echo ""

# Configuration
PROJECT_DIR="$HOME/autonomous-claude"
MASTER_IP="169.254.166.69"
WORKER_IP="169.254.114.16"
WORKER_HOST="claudia"
EMAIL_TO="robspierre19@gmail.com"
DASHBOARD_PORT=8080
QDRANT_PORT=6333

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() {
    echo -e "${GREEN}[$(date '+%H:%M:%S')]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[$(date '+%H:%M:%S')] WARNING:${NC} $1"
}

error() {
    echo -e "${RED}[$(date '+%H:%M:%S')] ERROR:${NC} $1"
}

# ============================================================================
# STEP 1: Create Directory Structure
# ============================================================================
log "Creating directory structure..."

mkdir -p "$PROJECT_DIR"/{data/memory,data/embeddings,projects,logs,screenshots,website/static,website/templates,skills}
mkdir -p "$PROJECT_DIR"/projects/{games,websites,art,stories,tools,experiments}

cd "$PROJECT_DIR"

# ============================================================================
# STEP 2: Install Dependencies via Homebrew
# ============================================================================
log "Installing dependencies via Homebrew..."

# Core tools
brew install sqlite 2>/dev/null || warn "sqlite already installed"
brew install node 2>/dev/null || warn "node already installed"
brew install python@3.12 2>/dev/null || warn "python already installed"

# Optional but useful
brew install imagemagick 2>/dev/null || warn "imagemagick already installed"
brew install ffmpeg 2>/dev/null || warn "ffmpeg already installed"
brew install jq 2>/dev/null || warn "jq already installed"

# Python packages
log "Installing Python packages..."
pip3 install --quiet qdrant-client sentence-transformers flask flask-socketio yagmail sqlite-utils mlx mlx-lm playwright 2>/dev/null

# Install Playwright browsers
python3 -m playwright install chromium 2>/dev/null || warn "Playwright chromium may already be installed"

# ============================================================================
# STEP 3: Initialize SQLite Database
# ============================================================================
log "Initializing SQLite short-term memory database..."

sqlite3 "$PROJECT_DIR/data/memory/short_term.db" << 'EOF'
CREATE TABLE IF NOT EXISTS memories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT DEFAULT (datetime('now')),
    type TEXT CHECK(type IN ('action', 'observation', 'thought', 'goal', 'error', 'milestone', 'idea')),
    content TEXT NOT NULL,
    node TEXT DEFAULT 'claude',
    importance INTEGER DEFAULT 5 CHECK(importance >= 1 AND importance <= 10)
);

CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT DEFAULT (datetime('now')),
    completed_at TEXT,
    name TEXT NOT NULL,
    type TEXT,
    path TEXT,
    status TEXT DEFAULT 'created' CHECK(status IN ('created', 'in_progress', 'completed', 'failed')),
    description TEXT,
    lines_of_code INTEGER DEFAULT 0,
    node TEXT DEFAULT 'claude'
);

CREATE TABLE IF NOT EXISTS hourly_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    hour INTEGER,
    timestamp TEXT DEFAULT (datetime('now')),
    projects_created INTEGER DEFAULT 0,
    total_loc INTEGER DEFAULT 0,
    memory_entries INTEGER DEFAULT 0,
    notable_events TEXT
);

CREATE TABLE IF NOT EXISTS email_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sent_at TEXT DEFAULT (datetime('now')),
    hour INTEGER,
    success INTEGER DEFAULT 1,
    error_message TEXT
);

CREATE INDEX IF NOT EXISTS idx_memories_timestamp ON memories(timestamp);
CREATE INDEX IF NOT EXISTS idx_memories_type ON memories(type);
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
CREATE INDEX IF NOT EXISTS idx_projects_type ON projects(type);
EOF

log "SQLite database initialized at $PROJECT_DIR/data/memory/short_term.db"

# ============================================================================
# STEP 4: Initialize Qdrant Vector Database
# ============================================================================
log "Setting up Qdrant vector database..."

# Download and setup Qdrant if not present
if ! command -v qdrant &> /dev/null; then
    log "Downloading Qdrant..."
    curl -L https://github.com/qdrant/qdrant/releases/latest/download/qdrant-aarch64-apple-darwin.tar.gz -o /tmp/qdrant.tar.gz
    tar -xzf /tmp/qdrant.tar.gz -C /usr/local/bin/
    rm /tmp/qdrant.tar.gz
fi

# Create Qdrant config
mkdir -p "$PROJECT_DIR/data/qdrant"
cat > "$PROJECT_DIR/data/qdrant/config.yaml" << EOF
storage:
  storage_path: $PROJECT_DIR/data/qdrant/storage
  
service:
  http_port: $QDRANT_PORT
  grpc_port: 6334
  
log_level: INFO
EOF

# Start Qdrant in background
log "Starting Qdrant..."
cd "$PROJECT_DIR/data/qdrant"
nohup qdrant --config-path config.yaml > "$PROJECT_DIR/logs/qdrant.log" 2>&1 &
QDRANT_PID=$!
echo $QDRANT_PID > "$PROJECT_DIR/data/qdrant/qdrant.pid"
sleep 3

# Initialize Qdrant collection
log "Creating Qdrant collection..."
python3 << 'PYEOF'
import time
from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams

# Wait for Qdrant to be ready
for i in range(10):
    try:
        client = QdrantClient(host="localhost", port=6333)
        client.get_collections()
        break
    except:
        time.sleep(1)
else:
    print("Warning: Qdrant may not be ready")

try:
    client.create_collection(
        collection_name="claude_memory",
        vectors_config=VectorParams(size=384, distance=Distance.COSINE)
    )
    print("Created claude_memory collection")
except Exception as e:
    if "already exists" in str(e):
        print("Collection already exists")
    else:
        print(f"Warning: {e}")
PYEOF

cd "$PROJECT_DIR"

# ============================================================================
# STEP 5: Create Dashboard Website
# ============================================================================
log "Creating dashboard website..."

# Flask app
cat > "$PROJECT_DIR/website/app.py" << 'PYEOF'
from flask import Flask, render_template, jsonify
from flask_socketio import SocketIO
import sqlite3
import os
from datetime import datetime
import json

app = Flask(__name__)
app.config['SECRET_KEY'] = 'autonomous-claude-secret'
socketio = SocketIO(app, cors_allowed_origins="*")

DB_PATH = os.path.expanduser("~/autonomous-claude/data/memory/short_term.db")
PROJECTS_DIR = os.path.expanduser("~/autonomous-claude/projects")
SCREENSHOTS_DIR = os.path.expanduser("~/autonomous-claude/screenshots")

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/api/stats')
def get_stats():
    conn = get_db()
    cur = conn.cursor()
    
    # Get project count
    cur.execute("SELECT COUNT(*) as count FROM projects")
    project_count = cur.fetchone()['count']
    
    # Get total LOC
    cur.execute("SELECT COALESCE(SUM(lines_of_code), 0) as total FROM projects")
    total_loc = cur.fetchone()['total']
    
    # Get memory count
    cur.execute("SELECT COUNT(*) as count FROM memories")
    memory_count = cur.fetchone()['count']
    
    # Get recent memories
    cur.execute("SELECT * FROM memories ORDER BY timestamp DESC LIMIT 20")
    memories = [dict(row) for row in cur.fetchall()]
    
    # Get recent projects
    cur.execute("SELECT * FROM projects ORDER BY created_at DESC LIMIT 10")
    projects = [dict(row) for row in cur.fetchall()]
    
    # Calculate uptime
    cur.execute("SELECT timestamp FROM memories WHERE type='milestone' ORDER BY timestamp ASC LIMIT 1")
    first_row = cur.fetchone()
    if first_row:
        start_time = datetime.fromisoformat(first_row['timestamp'])
        uptime = str(datetime.now() - start_time).split('.')[0]
    else:
        uptime = "00:00:00"
    
    conn.close()
    
    return jsonify({
        'project_count': project_count,
        'total_loc': total_loc,
        'memory_count': memory_count,
        'uptime': uptime,
        'memories': memories,
        'projects': projects,
        'timestamp': datetime.now().isoformat()
    })

@app.route('/api/projects')
def get_projects():
    conn = get_db()
    cur = conn.cursor()
    cur.execute("SELECT * FROM projects ORDER BY created_at DESC")
    projects = [dict(row) for row in cur.fetchall()]
    conn.close()
    return jsonify(projects)

@app.route('/api/screenshots')
def get_screenshots():
    screenshots = []
    if os.path.exists(SCREENSHOTS_DIR):
        for f in sorted(os.listdir(SCREENSHOTS_DIR), reverse=True)[:20]:
            if f.endswith('.png'):
                screenshots.append({
                    'filename': f,
                    'path': f'/static/screenshots/{f}'
                })
    return jsonify(screenshots)

@app.route('/portfolio')
def portfolio():
    return render_template('portfolio.html')

if __name__ == '__main__':
    socketio.run(app, host='0.0.0.0', port=8080, debug=False)
PYEOF

# HTML Template
cat > "$PROJECT_DIR/website/templates/index.html" << 'HTMLEOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Autonomous Claude - Live Dashboard</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/socket.io/4.0.1/socket.io.js"></script>
    <style>
        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }
        .pulse { animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite; }
        .terminal {
            font-family: 'Monaco', 'Menlo', monospace;
            background: #1a1a2e;
            color: #0f0;
        }
    </style>
</head>
<body class="bg-gray-900 text-white min-h-screen">
    <div class="container mx-auto px-4 py-8">
        <!-- Header -->
        <div class="text-center mb-8">
            <h1 class="text-4xl font-bold bg-gradient-to-r from-purple-400 to-pink-600 bg-clip-text text-transparent">
                🤖 Autonomous Claude
            </h1>
            <p class="text-gray-400 mt-2">24-Hour Experiment • claude + claudia Mac Studio Cluster</p>
            <div class="flex justify-center items-center mt-2 space-x-2">
                <span class="w-3 h-3 bg-green-500 rounded-full pulse"></span>
                <span class="text-green-400">RUNNING</span>
            </div>
        </div>

        <!-- Stats Grid -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
            <div class="bg-gray-800 rounded-lg p-4 text-center">
                <div class="text-3xl font-bold text-purple-400" id="projectCount">0</div>
                <div class="text-gray-400">Projects</div>
            </div>
            <div class="bg-gray-800 rounded-lg p-4 text-center">
                <div class="text-3xl font-bold text-blue-400" id="locCount">0</div>
                <div class="text-gray-400">Lines of Code</div>
            </div>
            <div class="bg-gray-800 rounded-lg p-4 text-center">
                <div class="text-3xl font-bold text-green-400" id="memoryCount">0</div>
                <div class="text-gray-400">Memories</div>
            </div>
            <div class="bg-gray-800 rounded-lg p-4 text-center">
                <div class="text-3xl font-bold text-yellow-400" id="uptime">00:00:00</div>
                <div class="text-gray-400">Uptime</div>
            </div>
        </div>

        <!-- Two Column Layout -->
        <div class="grid md:grid-cols-2 gap-6">
            <!-- Recent Activity -->
            <div class="bg-gray-800 rounded-lg p-4">
                <h2 class="text-xl font-bold mb-4 text-purple-400">📝 Recent Activity</h2>
                <div id="activityFeed" class="space-y-2 max-h-96 overflow-y-auto terminal p-3 rounded">
                    <p class="text-gray-500">Loading...</p>
                </div>
            </div>

            <!-- Recent Projects -->
            <div class="bg-gray-800 rounded-lg p-4">
                <h2 class="text-xl font-bold mb-4 text-blue-400">🚀 Recent Projects</h2>
                <div id="projectsFeed" class="space-y-2 max-h-96 overflow-y-auto">
                    <p class="text-gray-500">Loading...</p>
                </div>
            </div>
        </div>

        <!-- Footer -->
        <div class="text-center mt-8 text-gray-500">
            <p>Started: December 25, 2025 @ 11:20 AM CST</p>
            <p>Ends: December 26, 2025 @ 11:20 AM CST</p>
            <p class="mt-2"><a href="/portfolio" class="text-purple-400 hover:underline">View Portfolio →</a></p>
        </div>
    </div>

    <script>
        function updateStats() {
            fetch('/api/stats')
                .then(r => r.json())
                .then(data => {
                    document.getElementById('projectCount').textContent = data.project_count;
                    document.getElementById('locCount').textContent = data.total_loc.toLocaleString();
                    document.getElementById('memoryCount').textContent = data.memory_count;
                    document.getElementById('uptime').textContent = data.uptime;

                    // Update activity feed
                    const activityFeed = document.getElementById('activityFeed');
                    activityFeed.innerHTML = data.memories.map(m => 
                        `<div class="text-sm"><span class="text-gray-500">${m.timestamp}</span> <span class="text-yellow-400">[${m.type}]</span> ${m.content}</div>`
                    ).join('');

                    // Update projects feed
                    const projectsFeed = document.getElementById('projectsFeed');
                    projectsFeed.innerHTML = data.projects.map(p => 
                        `<div class="bg-gray-700 rounded p-2">
                            <div class="font-bold text-white">${p.name}</div>
                            <div class="text-sm text-gray-400">${p.type || 'unknown'} • ${p.lines_of_code || 0} LOC</div>
                        </div>`
                    ).join('');
                });
        }

        // Update every 5 seconds
        updateStats();
        setInterval(updateStats, 5000);
    </script>
</body>
</html>
HTMLEOF

# Portfolio template
cat > "$PROJECT_DIR/website/templates/portfolio.html" << 'HTMLEOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Autonomous Claude - Portfolio</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-900 text-white min-h-screen p-8">
    <div class="container mx-auto">
        <h1 class="text-4xl font-bold mb-8 text-center">🎨 Portfolio</h1>
        <div id="portfolio" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            Loading...
        </div>
        <div class="text-center mt-8">
            <a href="/" class="text-purple-400 hover:underline">← Back to Dashboard</a>
        </div>
    </div>
    <script>
        fetch('/api/projects')
            .then(r => r.json())
            .then(projects => {
                document.getElementById('portfolio').innerHTML = projects.map(p => `
                    <div class="bg-gray-800 rounded-lg p-4 hover:bg-gray-700 transition">
                        <h3 class="text-xl font-bold text-purple-400">${p.name}</h3>
                        <p class="text-gray-400 text-sm mt-1">${p.description || 'No description'}</p>
                        <div class="mt-2 text-sm">
                            <span class="text-blue-400">${p.type || 'unknown'}</span>
                            <span class="text-gray-500 ml-2">${p.lines_of_code || 0} LOC</span>
                        </div>
                    </div>
                `).join('');
            });
    </script>
</body>
</html>
HTMLEOF

# Symlink screenshots to static folder
ln -sf "$SCREENSHOTS_DIR" "$PROJECT_DIR/website/static/screenshots" 2>/dev/null || true

# ============================================================================
# STEP 6: Create Email Helper Script
# ============================================================================
log "Creating email helper..."

cat > "$PROJECT_DIR/send_update.py" << 'PYEOF'
#!/usr/bin/env python3
"""
Send hourly email update for Autonomous Claude experiment.
Usage: python3 send_update.py <hour_number>
"""

import sqlite3
import sys
import os
import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from datetime import datetime

DB_PATH = os.path.expanduser("~/autonomous-claude/data/memory/short_term.db")
EMAIL_TO = "robspierre19@gmail.com"

def get_stats():
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("SELECT COUNT(*) FROM projects")
    total_projects = cur.fetchone()[0]
    
    cur.execute("SELECT COALESCE(SUM(lines_of_code), 0) FROM projects")
    total_loc = cur.fetchone()[0]
    
    cur.execute("SELECT COUNT(*) FROM memories")
    memory_count = cur.fetchone()[0]
    
    cur.execute("SELECT content FROM memories WHERE type='thought' OR type='goal' ORDER BY timestamp DESC LIMIT 1")
    row = cur.fetchone()
    current_focus = row[0] if row else "Creating and exploring"
    
    cur.execute("SELECT name, description FROM projects ORDER BY created_at DESC LIMIT 5")
    recent = cur.fetchall()
    notable = "\n".join([f"  - {r[0]}: {r[1] or 'No description'}" for r in recent])
    
    cur.execute("SELECT content FROM memories WHERE type='lesson' OR type='discovery' ORDER BY timestamp DESC LIMIT 3")
    lessons = cur.fetchall()
    learnings = "\n".join([f"  - {l[0]}" for l in lessons]) or "  - Still learning..."
    
    conn.close()
    
    return {
        'total_projects': total_projects,
        'total_loc': total_loc,
        'memory_count': memory_count,
        'current_focus': current_focus,
        'notable': notable,
        'learnings': learnings
    }

def send_email(hour):
    stats = get_stats()
    
    subject = f"🤖 Autonomous Claude - Hour {hour} Update"
    body = f"""
Autonomous Claude Status Report
================================
Time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S CST')}
Hour: {hour} of 24

📊 Statistics:
- Total Projects: {stats['total_projects']}
- Lines of Code: {stats['total_loc']:,}
- Memory Entries: {stats['memory_count']}

🎯 Current Focus:
{stats['current_focus']}

✨ Notable Creations:
{stats['notable']}

🧠 Recent Learnings:
{stats['learnings']}

🔗 Dashboard: http://claude.local:8080

--
Autonomous Claude
Running on: claude + claudia Mac Studio Cluster
M3 Ultra × 2 | 1TB RAM | 18TB Storage
    """
    
    print(f"Hour {hour} update prepared:")
    print(body)
    print("\n[Note: Configure SMTP to actually send emails]")
    
    # Log the attempt
    conn = sqlite3.connect(DB_PATH)
    conn.execute("""
        INSERT INTO email_log (hour, success, error_message) 
        VALUES (?, 1, NULL)
    """, (hour,))
    conn.commit()
    conn.close()

if __name__ == "__main__":
    hour = int(sys.argv[1]) if len(sys.argv) > 1 else 1
    send_email(hour)
PYEOF

chmod +x "$PROJECT_DIR/send_update.py"

# ============================================================================
# STEP 7: Create Watcher Script (keeps Claude running)
# ============================================================================
log "Creating watcher script..."

cat > "$PROJECT_DIR/watcher.sh" << 'BASHEOF'
#!/bin/zsh
# Watcher script - restarts Claude Code if it exits

PROJECT_DIR="$HOME/autonomous-claude"
LOG_FILE="$PROJECT_DIR/logs/watcher.log"
HOUR=0
START_TIME=$(date +%s)
END_TIME=$((START_TIME + 86400))  # 24 hours

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

send_hourly_update() {
    HOUR=$((HOUR + 1))
    python3 "$PROJECT_DIR/send_update.py" "$HOUR"
    log "Sent hour $HOUR update"
}

run_claude() {
    cd "$PROJECT_DIR"
    log "Starting Claude Code session..."
    
    # Run Claude with the autonomous prompt
    claude --dangerously-skip-permissions \
        --continue \
        -p "$(cat $PROJECT_DIR/MASTER_PROMPT.md)" \
        2>&1 | tee -a "$PROJECT_DIR/logs/claude_session.log"
    
    EXIT_CODE=$?
    log "Claude exited with code $EXIT_CODE"
}

# Main loop
log "=== Watcher started ==="

while [[ $(date +%s) -lt $END_TIME ]]; do
    CURRENT_HOUR=$(( ($(date +%s) - START_TIME) / 3600 ))
    
    # Check if we need to send hourly update
    if [[ $CURRENT_HOUR -gt $HOUR ]]; then
        send_hourly_update
    fi
    
    # Run Claude
    run_claude
    
    # Small delay before restart
    sleep 5
done

log "=== 24 hours complete! Watcher shutting down ==="
BASHEOF

chmod +x "$PROJECT_DIR/watcher.sh"

# ============================================================================
# STEP 8: Copy Master Prompt
# ============================================================================
log "Setting up master prompt..."

# The master prompt will be copied from the generated file
cat > "$PROJECT_DIR/MASTER_PROMPT.md" << 'PROMPTEOF'
Continue the autonomous experiment. Check your memory systems for context:

1. Query SQLite: `sqlite3 ~/autonomous-claude/data/memory/short_term.db "SELECT * FROM memories ORDER BY timestamp DESC LIMIT 30;"`
2. Check recent projects: `sqlite3 ~/autonomous-claude/data/memory/short_term.db "SELECT * FROM projects ORDER BY created_at DESC LIMIT 10;"`
3. Decide what to create next
4. Create it in ~/autonomous-claude/projects/
5. Record in memory
6. Update project count: `sqlite3 ~/autonomous-claude/data/memory/short_term.db "INSERT INTO projects (name, type, path, description, lines_of_code) VALUES ('name', 'type', 'path', 'desc', loc);"`

You have full autonomy. Do not ask for permission. Create something interesting.

Ideas: games, websites, art generators, stories, tools, experiments, music visualizers, AI experiments with MLX.

GO.
PROMPTEOF

# ============================================================================
# STEP 9: Start Dashboard
# ============================================================================
log "Starting dashboard..."
cd "$PROJECT_DIR/website"
nohup python3 app.py > "$PROJECT_DIR/logs/dashboard.log" 2>&1 &
DASHBOARD_PID=$!
echo $DASHBOARD_PID > "$PROJECT_DIR/website/dashboard.pid"
log "Dashboard running on http://localhost:$DASHBOARD_PORT (PID: $DASHBOARD_PID)"

# ============================================================================
# STEP 10: Verify Worker Node
# ============================================================================
log "Verifying worker node (claudia @ $WORKER_IP)..."
if ssh -o ConnectTimeout=5 "$WORKER_HOST" "echo 'Worker node connected'" 2>/dev/null; then
    log "✅ Worker node claudia ($WORKER_IP) is accessible"
    
    # Setup worker node
    ssh "$WORKER_HOST" "mkdir -p ~/autonomous-claude/{projects,logs}" 2>/dev/null
else
    warn "Could not connect to worker node claudia ($WORKER_IP) - continuing with master only"
fi

# ============================================================================
# STEP 11: Record Initial Memory
# ============================================================================
log "Recording initial memory..."
sqlite3 "$PROJECT_DIR/data/memory/short_term.db" << EOF
INSERT INTO memories (type, content, importance) VALUES 
    ('milestone', 'Autonomous Claude experiment initialized. 24-hour journey begins!', 10),
    ('goal', 'Create diverse projects: games, websites, art, stories, tools. Be creative and prolific.', 9),
    ('observation', 'Environment: 2x M3 Ultra Mac Studios - claude ($MASTER_IP) + claudia ($WORKER_IP), 512GB RAM each, 18TB storage, Thunderbolt 5 RDMA', 8);
EOF

# ============================================================================
# COMPLETE
# ============================================================================
echo ""
echo "=============================================="
echo "🚀 BOOTSTRAP COMPLETE!"
echo "=============================================="
echo ""
echo "Cluster:"
echo "  • Master: claude @ $MASTER_IP"
echo "  • Worker: claudia @ $WORKER_IP"
echo ""
echo "Services running:"
echo "  • Dashboard: http://localhost:$DASHBOARD_PORT"
echo "  • Qdrant:    http://localhost:$QDRANT_PORT"
echo ""
echo "Project directory: $PROJECT_DIR"
echo ""
echo "=============================================="
echo "NEXT STEP: Start Claude Code with your prompt"
echo "=============================================="
echo ""
echo "  cd $PROJECT_DIR"
echo "  claude --dangerously-skip-permissions"
echo ""
echo "Then paste the contents of START-HERE-PASTE-INTO-CLAUDE-CODE.md"
echo ""
echo "OR run the watcher for auto-restart:"
echo ""
echo "  cd $PROJECT_DIR && ./watcher.sh"
echo ""
echo "=============================================="

