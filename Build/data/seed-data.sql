INSERT OR IGNORE INTO packages (id, name) VALUES
    (1, 'ffmpeg'),
    (2, 'python'),
    (3, 'node'),
    (4, 'git'),
    (5, 'curl'),
    (6, 'vim'),
    (7, 'libpng'),
    (8, 'openssl'),
    (9, 'make'),
    (10, 'gcc'),
    (11, 'ripgrep'),
    (12, 'jq'),
    (13, 'htop'),
    (14, 'wget'),
    (15, 'unzip'),
    (16, 'tmux'),
    (17, 'zlib'),
    (18, 'readline'),
    (19, 'sqlite'),
    (20, 'libxml2');

-- macOS (Homebrew): target: Macos
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) VALUES
    (1,  'Macos', 'ffmpeg',              'manual', 1.0, 'ffmpeg'),
    (2,  'Macos', 'python',              'manual', 1.0, 'python'),
    (3,  'Macos', 'node',                'manual', 1.0, 'node'),
    (4,  'Macos', 'git',                 'manual', 1.0, 'git'),
    (5,  'Macos', 'curl',                'manual', 1.0, 'curl'),
    (6,  'Macos', 'vim',                 'manual', 1.0, 'vim'),
    (7,  'Macos', 'libpng',              'manual', 1.0, 'libpng'),
    (8,  'Macos', 'openssl',             'manual', 1.0, 'openssl'),
    (9,  'Macos', 'make',                'manual', 1.0, 'make'),
    (10, 'Macos', 'gcc',                 'manual', 1.0, 'gcc'),
    (11, 'Macos', 'ripgrep',             'manual', 1.0, 'ripgrep'),
    (12, 'Macos', 'jq',                  'manual', 1.0, 'jq'),
    (13, 'Macos', 'htop',                'manual', 1.0, 'htop'),
    (14, 'Macos', 'wget',                'manual', 1.0, 'wget'),
    (15, 'Macos', 'unzip',               'manual', 1.0, 'unzip'),
    (16, 'Macos', 'tmux',                'manual', 1.0, 'tmux'),
    (17, 'Macos', 'zlib',                'manual', 1.0, 'zlib'),
    (18, 'Macos', 'readline',            'manual', 1.0, 'readline'),
    (19, 'Macos', 'sqlite',              'manual', 1.0, 'sqlite'),
    (20, 'Macos', 'libxml2',             'manual', 1.0, 'libxml2');

-- Debian-family (apt): targets: Debian, Ubuntu, Mint, Pop, Elementary, Zorin, Kali, Raspbian
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) VALUES
    (1,  'Debian',    'ffmpeg',            'manual', 1.0, 'ffmpeg'),
    (2,  'Debian',    'python3',           'manual', 1.0, 'python3 is the system python'),
    (3,  'Debian',    'nodejs',            'manual', 1.0, 'nodejs (not node)'),
    (4,  'Debian',    'git',               'manual', 1.0, 'git'),
    (5,  'Debian',    'curl',              'manual', 1.0, 'curl'),
    (6,  'Debian',    'vim',               'manual', 1.0, 'vim'),
    (7,  'Debian',    'libpng-dev',        'manual', 1.0, 'libpng-dev'),
    (8,  'Debian',    'libssl-dev',        'manual', 1.0, 'libssl-dev'),
    (9,  'Debian',    'make',              'manual', 1.0, 'make'),
    (10, 'Debian',    'gcc',               'manual', 1.0, 'gcc'),
    (11, 'Debian',    'ripgrep',           'manual', 1.0, 'ripgrep'),
    (12, 'Debian',    'jq',                'manual', 1.0, 'jq'),
    (13, 'Debian',    'htop',              'manual', 1.0, 'htop'),
    (14, 'Debian',    'wget',              'manual', 1.0, 'wget'),
    (15, 'Debian',    'unzip',             'manual', 1.0, 'unzip'),
    (16, 'Debian',    'tmux',              'manual', 1.0, 'tmux'),
    (17, 'Debian',    'zlib1g-dev',        'manual', 1.0, 'zlib1g-dev'),
    (18, 'Debian',    'libreadline-dev',   'manual', 1.0, 'libreadline-dev'),
    (19, 'Debian',    'libsqlite3-dev',    'manual', 1.0, 'libsqlite3-dev'),
    (20, 'Debian',    'libxml2-dev',       'manual', 1.0, 'libxml2-dev');

-- Ubuntu (apt): same package names as Debian, explicit entry
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Ubuntu' AS os, os_package, source, confidence, notes || ' (Ubuntu)' FROM mappings WHERE os = 'Debian';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Mint' AS os, os_package, source, confidence, notes || ' (Mint)' FROM mappings WHERE os = 'Debian';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Pop' AS os, os_package, source, confidence, notes || ' (Pop)' FROM mappings WHERE os = 'Debian';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Elementary' AS os, os_package, source, confidence, notes || ' (Elementary)' FROM mappings WHERE os = 'Debian';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Zorin' AS os, os_package, source, confidence, notes || ' (Zorin)' FROM mappings WHERE os = 'Debian';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Kali' AS os, os_package, source, confidence, notes || ' (Kali)' FROM mappings WHERE os = 'Debian';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Raspbian' AS os, os_package, source, confidence, notes || ' (Raspbian)' FROM mappings WHERE os = 'Debian';

-- Fedora-family (dnf): targets: Fedora, Nobara, Ultramarine
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) VALUES
    (1,  'Fedora',    'ffmpeg',            'manual', 1.0, 'ffmpeg'),
    (2,  'Fedora',    'python3',           'manual', 1.0, 'python3'),
    (3,  'Fedora',    'nodejs',            'manual', 1.0, 'nodejs'),
    (4,  'Fedora',    'git',               'manual', 1.0, 'git'),
    (5,  'Fedora',    'curl',              'manual', 1.0, 'curl'),
    (6,  'Fedora',    'vim-enhanced',      'manual', 1.0, 'vim-enhanced'),
    (7,  'Fedora',    'libpng-devel',      'manual', 1.0, 'libpng-devel'),
    (8,  'Fedora',    'openssl-devel',     'manual', 1.0, 'openssl-devel'),
    (9,  'Fedora',    'make',              'manual', 1.0, 'make'),
    (10, 'Fedora',    'gcc',               'manual', 1.0, 'gcc'),
    (11, 'Fedora',    'ripgrep',           'manual', 1.0, 'ripgrep'),
    (12, 'Fedora',    'jq',                'manual', 1.0, 'jq'),
    (13, 'Fedora',    'htop',              'manual', 1.0, 'htop'),
    (14, 'Fedora',    'wget',              'manual', 1.0, 'wget'),
    (15, 'Fedora',    'unzip',             'manual', 1.0, 'unzip'),
    (16, 'Fedora',    'tmux',              'manual', 1.0, 'tmux'),
    (17, 'Fedora',    'zlib-devel',        'manual', 1.0, 'zlib-devel'),
    (18, 'Fedora',    'readline-devel',    'manual', 1.0, 'readline-devel'),
    (19, 'Fedora',    'sqlite-devel',      'manual', 1.0, 'sqlite-devel'),
    (20, 'Fedora',    'libxml2-devel',     'manual', 1.0, 'libxml2-devel');

INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Nobara' AS os, os_package, source, confidence, notes || ' (Nobara)' FROM mappings WHERE os = 'Fedora';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Ultramarine' AS os, os_package, source, confidence, notes || ' (Ultramarine)' FROM mappings WHERE os = 'Fedora';

-- Arch-family (pacman): targets: Arch, Manjaro, EndeavourOS, Artix, Garuda, CachyOS
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) VALUES
    (1,  'Arch',      'ffmpeg',            'manual', 1.0, 'ffmpeg'),
    (2,  'Arch',      'python',            'manual', 1.0, 'python (not python3)'),
    (3,  'Arch',      'nodejs',            'manual', 1.0, 'nodejs'),
    (4,  'Arch',      'git',               'manual', 1.0, 'git'),
    (5,  'Arch',      'curl',              'manual', 1.0, 'curl'),
    (6,  'Arch',      'gvim',              'manual', 1.0, 'gvim (includes CLI vim)'),
    (7,  'Arch',      'libpng',            'manual', 1.0, 'libpng'),
    (8,  'Arch',      'openssl',           'manual', 1.0, 'openssl'),
    (9,  'Arch',      'make',              'manual', 1.0, 'make'),
    (10, 'Arch',      'gcc',               'manual', 1.0, 'gcc'),
    (11, 'Arch',      'ripgrep',           'manual', 1.0, 'ripgrep'),
    (12, 'Arch',      'jq',                'manual', 1.0, 'jq'),
    (13, 'Arch',      'htop',              'manual', 1.0, 'htop'),
    (14, 'Arch',      'wget',              'manual', 1.0, 'wget'),
    (15, 'Arch',      'unzip',             'manual', 1.0, 'unzip'),
    (16, 'Arch',      'tmux',              'manual', 1.0, 'tmux'),
    (17, 'Arch',      'zlib',              'manual', 1.0, 'zlib'),
    (18, 'Arch',      'readline',          'manual', 1.0, 'readline'),
    (19, 'Arch',      'sqlite',            'manual', 1.0, 'sqlite'),
    (20, 'Arch',      'libxml2',           'manual', 1.0, 'libxml2');

INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Manjaro' AS os, os_package, source, confidence, notes || ' (Manjaro)' FROM mappings WHERE os = 'Arch';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'EndeavourOS' AS os, os_package, source, confidence, notes || ' (EndeavourOS)' FROM mappings WHERE os = 'Arch';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Artix' AS os, os_package, source, confidence, notes || ' (Artix)' FROM mappings WHERE os = 'Arch';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'Garuda' AS os, os_package, source, confidence, notes || ' (Garuda)' FROM mappings WHERE os = 'Arch';
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) SELECT package_id, 'CachyOS' AS os, os_package, source, confidence, notes || ' (CachyOS)' FROM mappings WHERE os = 'Arch';

-- Windows (winget / chocolatey): target: Windows
INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence, notes) VALUES
    (1,  'Windows',   'FFmpeg',             'manual', 1.0, 'FFmpeg (winget)'),
    (2,  'Windows',   'Python.Python',      'manual', 1.0, 'Python.Python.3'),
    (3,  'Windows',   'OpenJS.NodeJS',      'manual', 1.0, 'OpenJS.NodeJS'),
    (4,  'Windows',   'Git.Git',            'manual', 1.0, 'Git.Git'),
    (5,  'Windows',   'curl',               'manual', 1.0, 'curl (alias)'),
    (6,  'Windows',   'vim',                'manual', 1.0, 'vim'),
    (7,  'Windows',   'libpng',             'manual', 1.0, 'libpng'),
    (8,  'Windows',   'OpenSSL.Light',      'manual', 1.0, 'OpenSSL.Light'),
    (9,  'Windows',   'GnuWin32.Make',      'manual', 1.0, 'GnuWin32.Make'),
    (10, 'Windows',   'GnuWin32.GCC',       'manual', 1.0, 'GnuWin32.GCC'),
    (11, 'Windows',   'BurntSushi.ripgrep', 'manual', 1.0, 'BurntSushi.ripgrep'),
    (12, 'Windows',   'jq',                 'manual', 1.0, 'jq'),
    (13, 'Windows',   'htop',               'manual', 0.7, 'no direct winget package, low confidence'),
    (14, 'Windows',   'GnuWin32.Wget',      'manual', 1.0, 'GnuWin32.Wget'),
    (15, 'Windows',   'unzip',              'manual', 1.0, 'unzip'),
    (16, 'Windows',   'tmux',               'manual', 0.5, 'no direct winget package, very low confidence'),
    (17, 'Windows',   'zlib',               'manual', 1.0, 'zlib'),
    (18, 'Windows',   'readline',           'manual', 0.5, 'no direct winget package'),
    (19, 'Windows',   'SQLite',             'manual', 1.0, 'SQLite'),
    (20, 'Windows',   'libxml2',            'manual', 0.5, 'no direct winget package');

INSERT OR IGNORE INTO aliases (alias, canonical, source, notes) VALUES
    ('python3',  'python',     'manual', 'Debian-style python3 alias'),
    ('nodejs',   'node',       'manual', 'Debian-style nodejs alias'),
    ('pip',      'python-pip', 'manual', 'pip is python-pip in many distros'),
    ('gcc',      'gcc-defaults', 'manual', 'gcc is gcc-defaults on Debian-family'),
    ('g++',      'gcc-defaults', 'manual', 'g++ is also gcc-defaults'),
    ('libpng-dev', 'libpng',   'manual', '-dev variant'),
    ('libssl-dev', 'openssl',  'manual', '-dev variant'),
    ('zlib1g-dev', 'zlib',     'manual', 'Debian-specific naming'),
    ('libxml2-dev', 'libxml2', 'manual', '-dev variant'),
    ('sqlite3',  'sqlite',     'manual', 'sqlite3 is the common name'),
    ('vim-tiny', 'vim',        'manual', 'minimal vim variant'),
    ('nano',     'nano',       'manual', 'identity alias');
