import GmailService from '../services/GmailService.js';
import logger from '../utils/logger.js';

/**
 * VirtualComputer - Simulates a PC environment for agents in their digital world
 * Provides Gmail access, Chrome browser, terminal, and notepad functionality
 */
class VirtualComputer {
  constructor(ownerId, ownerName) {
    this.ownerId = ownerId; // 'gem-d' or 'gem-k'
    this.ownerName = ownerName;
    this.poweredOn = true;
    this.applications = {
      chrome: new ChromeBrowser(this),
      terminal: new Terminal(this),
      notepad: new Notepad(this),
      fileManager: new FileManager(this)
    };

    // Gmail accounts with API integration
    this.gmailAccount = {
      email: ownerId === 'gem-d' ? 'gemdtwin98@gmail.com' : 'gemktwin91@gmail.com',
      password: 'KDgemini@25',
      clientId: ownerId === 'gem-d'
        ? '469229413414-v82119uc9va6i87vthe0ev05qvgj8888.apps.googleusercontent.com'
        : '360027666424-eq9h6205n6k74qu8cgqnbdtg17agsg5k.apps.googleusercontent.com',
      clientSecret: ownerId === 'gem-d'
        ? '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'
        : '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',
      isLoggedIn: true,
      contacts: [
        { name: 'Dylan', email: 'dylanmarriner@yahoo.com' },
        { name: 'Kirsty', email: 'kirstygainfort@gmail.com' }
      ],
      inbox: [],
      sent: []
    };

    // File system
    this.fileSystem = new FileSystem(this);

    // Network connectivity
    this.network = {
      connected: true,
      ipAddress: `192.168.1.${ownerId === 'gem-d' ? '101' : '102'}`,
      dns: '8.8.8.8',
      internetAccess: true
    };

    // System info
    this.systemInfo = {
      os: 'GeminiOS v2.0',
      hostname: `${ownerId}-pc`,
      uptime: 0,
      lastBoot: new Date()
    };

    // Initialize with some default files
    this._initializeDefaultFiles();
  }

  /**
   * Get complete computer state for UI transparency
   */
  getComputerState() {
    return {
      ownerId: this.ownerId,
      ownerName: this.ownerName,
      poweredOn: this.poweredOn,
      activeApplications: this._getActiveApplications(),
      desktop: {
        wallpaper: '/assets/desktop-gemini.jpg',
        icons: [
          { id: 'chrome', name: 'Chrome Browser', icon: '🌐', running: this.applications.chrome.isRunning },
          { id: 'terminal', name: 'Terminal', icon: '💻', running: this.applications.terminal.isRunning },
          { id: 'notepad', name: 'Notepad', icon: '📝', running: this.applications.notepad.isRunning },
          { id: 'files', name: 'File Manager', icon: '📁', running: this.applications.fileManager.isRunning }
        ]
      },
      gmail: {
        ...this.gmailAccount,
        unreadCount: this.gmailAccount.inbox.filter(m => !m.read).length
      },
      network: this.network,
      systemInfo: {
        ...this.systemInfo,
        uptime: this._calculateUptime()
      },
      openWindows: this._getOpenWindows()
    };
  }

  /**
   * Launch an application
   */
  launchApplication(appName) {
    const app = this.applications[appName];
    if (!app) {
      throw new Error(`Application ${appName} not found`);
    }

    if (!this.poweredOn) {
      throw new Error('Computer is powered off');
    }

    app.launch();
    return app.getState();
  }

  /**
   * Close an application
   */
  closeApplication(appName) {
    const app = this.applications[appName];
    if (app && app.isRunning) {
      app.close();
    }
  }

  /**
   * Send email via Gmail API
   */
  async sendEmail(to, subject, body) {
    if (!this.gmailAccount.isLoggedIn) {
      throw new Error('Not logged into Gmail');
    }

    if (!this.network.internetAccess) {
      throw new Error('No internet connection');
    }

    try {
      // Use GmailService to send actual email
      const result = await GmailService.sendEmail(this.ownerId, to, subject, body);

      // Store in sent items for tracking
      const email = {
        id: result.id,
        from: this.gmailAccount.email,
        to,
        subject,
        body,
        timestamp: new Date(),
        sent: true,
        messageId: result.id
      };

      this.gmailAccount.sent.push(email);

      logger.info(`[${this.ownerId}] Email sent via Gmail API to ${to}: ${subject}`);

      return email;
    } catch (error) {
      logger.error(`[${this.ownerId}] Gmail API error:`, error);

      // Fallback: queue email for retry instead of using mock data
      logger.warn(`[${this.ownerId}] Email send failed, queuing for retry: ${to}`);
      
      const email = {
        id: `retry_${Date.now()}_${require('crypto').randomUUID().replace(/-/g, '').substring(0, 9)}`,
        from: this.gmailAccount.email,
        to,
        subject,
        body,
        timestamp: new Date(),
        sent: false,
        queued_for_retry: true,
        error: error.message,
        retry_count: 0,
        max_retries: 3
      };

      // Queue for later retry instead of marking as sent with mock data
      if (!this.gmailAccount.queue) {
        this.gmailAccount.queue = [];
      }
      this.gmailAccount.queue.push(email);

      return email;
    }
  }

  /**
   * Check for new emails via Gmail API
   */
  async checkEmail() {
    if (!this.gmailAccount.isLoggedIn || !this.network.internetAccess) {
      return [];
    }

    try {
      // Use GmailService to get actual emails
      const newEmails = await GmailService.getRecentEmails(this.ownerId, 10);

      // Add to inbox if not already present
      newEmails.forEach(email => {
        if (!this.gmailAccount.inbox.find(e => e.id === email.id)) {
          this.gmailAccount.inbox.push({
            ...email,
            timestamp: new Date(email.date),
            read: false
          });
        }
      });

      return newEmails;
    } catch (error) {
      logger.error(`[${this.ownerId}] Gmail check error:`, error);
      
      // Return empty array instead of mock data
      // System should handle gracefully when no emails are available
      logger.warn(`[${this.ownerId}] Gmail API unavailable, returning empty email list`);
      return [];
    }
  }

  /**
   * Mark email as read
   */
  async markEmailAsRead(messageId) {
    try {
      await GmailService.markAsRead(this.ownerId, messageId);

      // Update local inbox
      const email = this.gmailAccount.inbox.find(e => e.id === messageId);
      if (email) {
        email.read = true;
      }
    } catch (error) {
      logger.error(`[${this.ownerId}] Failed to mark email as read:`, error);
    }
  }

  /**
   * Power on/off the computer
   */
  setPower(on) {
    this.poweredOn = on;
    if (!on) {
      // Close all applications
      Object.values(this.applications).forEach(app => {
        if (app.isRunning) app.close();
      });
    } else {
      this.systemInfo.lastBoot = new Date();
    }
  }

  /**
   * Initialize default files
   */
  _initializeDefaultFiles() {
    this.fileSystem.createFile('/home/README.txt', `Welcome to ${this.ownerName}'s computer!`);
    this.fileSystem.createFile('/home/contacts.txt',
      'Dylan: dylanmarriner@yahoo.com\nKirsty: kirstygainfort@gmail.com'
    );
    this.fileSystem.createFile('/home/notes.txt', 'System notes:\n- Gmail configured\n- Chrome ready\n- Terminal accessible');
  }

  /**
   * Get list of active applications
   */
  _getActiveApplications() {
    return Object.entries(this.applications)
      .filter(([_, app]) => app.isRunning)
      .map(([name, app]) => ({ name, ...app.getState() }));
  }

  /**
   * Get open windows for UI
   */
  _getOpenWindows() {
    return Object.entries(this.applications)
      .filter(([_, app]) => app.isRunning)
      .map(([name, app]) => ({
        id: name,
        title: app.title,
        content: app.getDisplayContent(),
        position: app.windowPosition || { x: 100, y: 100 },
        size: app.windowSize || { width: 800, height: 600 }
      }));
  }

  /**
   * Calculate system uptime
   */
  _calculateUptime() {
    return Date.now() - this.systemInfo.lastBoot.getTime();
  }
}

/**
 * Chrome Browser Application
 */
class ChromeBrowser {
  constructor(computer) {
    this.computer = computer;
    this.isRunning = false;
    this.title = 'Google Chrome';
    this.tabs = [
      {
        id: 'tab_1',
        title: 'Gmail',
        url: 'https://mail.google.com',
        active: true
      }
    ];
    this.bookmarks = [
      { title: 'Gmail', url: 'https://mail.google.com' },
      { title: 'Google', url: 'https://google.com' },
      { title: 'GitHub', url: 'https://github.com' }
    ];
  }

  launch() {
    this.isRunning = true;
  }

  close() {
    this.isRunning = false;
  }

  getState() {
    return {
      isRunning: this.isRunning,
      title: this.title,
      tabs: this.tabs,
      activeTab: this.tabs.find(t => t.active),
      bookmarks: this.bookmarks,
      isLoggedIn: this.computer.gmailAccount.isLoggedIn
    };
  }

  getDisplayContent() {
    const activeTab = this.tabs.find(t => t.active);
    return {
      type: 'browser',
      url: activeTab?.url || 'about:blank',
      title: activeTab?.title || 'New Tab',
      content: activeTab?.url === 'https://mail.google.com' ?
        this._renderGmailInterface() :
        '<div>Browser content placeholder</div>'
    };
  }

  _renderGmailInterface() {
    const { inbox } = this.computer.gmailAccount;
    return `
      <div class="gmail-interface">
        <div class="gmail-sidebar">
          <div class="gmail-compose">Compose</div>
          <div class="gmail-label">Inbox (${inbox.filter(m => !m.read).length})</div>
          <div class="gmail-label">Sent</div>
          <div class="gmail-label">Drafts</div>
        </div>
        <div class="gmail-content">
          <div class="gmail-toolbar">
            <button>Refresh</button>
            <button>Search</button>
          </div>
          <div class="gmail-list">
            ${inbox.map(email => `
              <div class="gmail-item ${email.read ? 'read' : 'unread'}">
                <div class="gmail-from">${email.from}</div>
                <div class="gmail-subject">${email.subject}</div>
                <div class="gmail-preview">${email.body.substring(0, 50)}...</div>
              </div>
            `).join('')}
          </div>
        </div>
      </div>
    `;
  }
}

/**
 * Terminal Application
 */
class Terminal {
  constructor(computer) {
    this.computer = computer;
    this.isRunning = false;
    this.title = 'Terminal';
    this.history = [
      `$ Welcome to ${computer.ownerName}'s terminal`,
      `$ Type 'help' for available commands`,
      `$ `
    ];
    this.currentCommand = '';
    this.cwd = '/home';
  }

  launch() {
    this.isRunning = true;
  }

  close() {
    this.isRunning = false;
  }

  getState() {
    return {
      isRunning: this.isRunning,
      title: this.title,
      history: this.history,
      cwd: this.cwd
    };
  }

  getDisplayContent() {
    return {
      type: 'terminal',
      content: this.history.join('\n'),
      prompt: `$ ${this.cwd}> `
    };
  }

  executeCommand(command) {
    this.history[this.history.length - 1] = `$ ${command}`;

    const output = this._processCommand(command);
    this.history.push(...output);
    this.history.push(`$ `);

    return output;
  }

  _processCommand(command) {
    const cmd = command.trim().split(' ')[0];
    const args = command.trim().split(' ').slice(1);

    switch (cmd) {
      case 'help':
        return [
          'Available commands:',
          '  help     - Show this help',
          '  ls       - List files',
          '  cat      - Display file contents',
          '  gmail    - Check Gmail',
          '  chrome   - Open Chrome',
          '  clear    - Clear terminal'
        ];
      case 'ls':
        return ['README.txt', 'contacts.txt', 'notes.txt'];
      case 'cat':
        if (args[0]) {
          const file = this.computer.fileSystem.readFile(`${this.cwd}/${args[0]}`);
          return file ? [file.content] : [`File not found: ${args[0]}`];
        }
        return ['Usage: cat <filename>'];
      case 'gmail':
        return [`Checking Gmail for ${this.computer.gmailAccount.email}...`,
        `${this.computer.gmailAccount.inbox.length} messages in inbox`];
      case 'clear':
        this.history = [`$ `];
        return [];
      default:
        return [`Command not found: ${cmd}`];
    }
  }
}

/**
 * Notepad Application
 */
class Notepad {
  constructor(computer) {
    this.computer = computer;
    this.isRunning = false;
    this.title = 'Notepad';
    this.content = '';
    this.currentFile = null;
  }

  launch() {
    this.isRunning = true;
  }

  close() {
    this.isRunning = false;
  }

  getState() {
    return {
      isRunning: this.isRunning,
      title: this.title,
      content: this.content,
      currentFile: this.currentFile
    };
  }

  getDisplayContent() {
    return {
      type: 'notepad',
      content: this.content,
      filename: this.currentFile || 'Untitled'
    };
  }

  setText(text) {
    this.content = text;
  }

  saveFile(filename) {
    this.computer.fileSystem.createFile(`/home/${filename}`, this.content);
    this.currentFile = filename;
  }

  openFile(filename) {
    const file = this.computer.fileSystem.readFile(`/home/${filename}`);
    if (file) {
      this.content = file.content;
      this.currentFile = filename;
    }
  }
}

/**
 * File Manager Application
 */
class FileManager {
  constructor(computer) {
    this.computer = computer;
    this.isRunning = false;
    this.title = 'File Manager';
    this.currentPath = '/home';
  }

  launch() {
    this.isRunning = true;
  }

  close() {
    this.isRunning = false;
  }

  getState() {
    return {
      isRunning: this.isRunning,
      title: this.title,
      currentPath: this.currentPath,
      files: this.computer.fileSystem.listFiles(this.currentPath)
    };
  }

  getDisplayContent() {
    const files = this.computer.fileSystem.listFiles(this.currentPath);
    return {
      type: 'file-manager',
      path: this.currentPath,
      files: files.map(f => ({
        name: f.name,
        type: f.type,
        size: f.size,
        modified: f.modified
      }))
    };
  }
}

/**
 * Simple File System for virtual computer
 */
class FileSystem {
  constructor(computer) {
    this.computer = computer;
    this.files = new Map();
    this.directories = new Set(['/home']);
  }

  createFile(path, content) {
    const parts = path.split('/');
    const filename = parts.pop();
    const directory = parts.join('/');

    this.files.set(path, {
      name: filename,
      content,
      type: 'file',
      size: content.length,
      created: new Date(),
      modified: new Date()
    });

    this.directories.add(directory);
  }

  readFile(path) {
    return this.files.get(path);
  }

  listFiles(directory) {
    return Array.from(this.files.entries())
      .filter(([path, _]) => path.startsWith(directory + '/'))
      .map(([_, file]) => file);
  }
}

export default VirtualComputer;
