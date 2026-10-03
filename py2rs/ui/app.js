const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;
const $ = (id) => document.getElementById(id);
let T = {}, root = "", files = [];

async function setLang(l) {
  T = await (await fetch(`i18n/${l}.json`)).json();
  document.querySelectorAll("[data-t]").forEach((e) => (e.textContent = T[e.dataset.t]));
  $("desc").placeholder = T.aboutHint;
  $("docText").placeholder = T.docPlaceholder;
}
$("lang").onchange = (e) => setLang(e.target.value);
setLang("ru");

async function checkFirstRun() {
  const isFirst = await invoke("is_first_run").catch(() => true);
  if (isFirst) {
    $("setupDialog").showModal();
  }
}

$("setupKind").onchange = (e) => {
  $("setupBaseRow").style.display = ["anthropic"].includes(e.target.value) ? "none" : "block";
};

$("setupSave").onclick = async () => {
  const provider = {
    kind: $("setupKind").value,
    base_url: $("setupBase").value,
    api_key: $("setupKey").value || null,
    model: $("setupModel").value
  };

  await invoke("save_first_run_config", { provider });

  $("kind").value = provider.kind;
  $("base").value = provider.base_url;
  $("model").value = provider.model;
  $("key").value = provider.api_key || "";

  $("setupDialog").close();
};

checkFirstRun();

$("kind").onchange = async (e) => {
  const val = e.target.value;
  $("baseRow").style.display = ["anthropic"].includes(val) ? "none" : "block";
  $("customFields").style.display = val === "custom" ? "block" : "none";
  $("key").value = (await invoke("load_key", { provider: val })) ?? "";
};
$("kind").dispatchEvent(new Event("change"));
$("key").onblur = () => invoke("save_key", { provider: $("kind").value, key: $("key").value }).catch(() => {});

let libs = [];
let docsMap = {};
let selectedFiles = new Set();
const cratesBox = $("crates");

$("pick").onclick = async () => {
  const dir = await open({ directory: true });
  if (!dir) return;
  root = dir; $("path").textContent = dir;

  const respectGitignore = $("respectGitignore").checked;
  files = await invoke("plan_project", { root, respectGitignore });
  $("count").textContent = files.length;

  // Initialize file selection - all files selected by default
  selectedFiles.clear();
  files.forEach((_, idx) => selectedFiles.add(idx));

  // Populate file list
  const container = $("fileListContainer");
  container.innerHTML = "";
  files.forEach((f, idx) => {
    const label = document.createElement("label");
    label.style.display = "block";
    label.style.padding = "4px 0";
    label.innerHTML = `<input type="checkbox" data-idx="${idx}" checked> <span style="font-family:monospace;font-size:14px">${f.file}</span>`;
    container.appendChild(label);
  });
  $("fileList").style.display = "block";

  libs = await invoke("detect_libs", { root });
  docsMap = {};
  cratesBox.innerHTML = "";
  for (const { py, suggestion, doc } of libs) {
    docsMap[py] = doc ?? "";
    const row = document.createElement("label");
    row.className = "crate-row";
    row.innerHTML = `<span>${py}</span><input data-py="${py}" value="${suggestion ?? ""}" placeholder="${T.skipCrate}"><button type="button" data-doc="${py}" title="${T.docTitle}">📝</button>`;
    cratesBox.appendChild(row);
  }
};

// Drag and drop support
const dropZone = $("dropZone");
const dropOverlay = $("dropOverlay");

dropZone.addEventListener("dragover", (e) => {
  e.preventDefault();
  e.stopPropagation();
  dropOverlay.style.display = "flex";
});

dropZone.addEventListener("dragleave", (e) => {
  e.preventDefault();
  e.stopPropagation();
  if (e.target === dropZone) {
    dropOverlay.style.display = "none";
  }
});

dropZone.addEventListener("drop", async (e) => {
  e.preventDefault();
  e.stopPropagation();
  dropOverlay.style.display = "none";

  const items = e.dataTransfer.items;
  if (items && items.length > 0) {
    for (let i = 0; i < items.length; i++) {
      const item = items[i];
      if (item.kind === "file") {
        const entry = item.webkitGetAsEntry();
        if (entry && entry.isDirectory) {
          // Trigger folder selection with the dropped folder
          try {
            const path = await invoke("get_dropped_folder_path", { entry: entry.fullPath });
            if (path) {
              root = path;
              $("path").textContent = path;

              const respectGitignore = $("respectGitignore").checked;
              files = await invoke("plan_project", { root: path, respectGitignore });
              $("count").textContent = files.length;

              selectedFiles.clear();
              files.forEach((_, idx) => selectedFiles.add(idx));

              const container = $("fileListContainer");
              container.innerHTML = "";
              files.forEach((f, idx) => {
                const label = document.createElement("label");
                label.style.display = "block";
                label.style.padding = "4px 0";
                label.innerHTML = `<input type="checkbox" data-idx="${idx}" checked> <span style="font-family:monospace;font-size:14px">${f.file}</span>`;
                container.appendChild(label);
              });
              $("fileList").style.display = "block";

              libs = await invoke("detect_libs", { root: path });
              docsMap = {};
              cratesBox.innerHTML = "";
              for (const { py, suggestion, doc } of libs) {
                docsMap[py] = doc ?? "";
                const row = document.createElement("label");
                row.className = "crate-row";
                row.innerHTML = `<span>${py}</span><input data-py="${py}" value="${suggestion ?? ""}" placeholder="${T.skipCrate}"><button type="button" data-doc="${py}" title="${T.docTitle}">📝</button>`;
                cratesBox.appendChild(row);
              }
            }
          } catch (err) {
            log(`Error processing dropped folder: ${err}`);
          }
          break;
        }
      }
    }
  }
});

// File selection handlers
$("fileListContainer").onchange = (e) => {
  if (e.target.type === "checkbox") {
    const idx = parseInt(e.target.dataset.idx, 10);
    if (e.target.checked) {
      selectedFiles.add(idx);
    } else {
      selectedFiles.delete(idx);
    }
    $("count").textContent = selectedFiles.size;
  }
};

$("selectAll").onclick = () => {
  selectedFiles.clear();
  files.forEach((_, idx) => selectedFiles.add(idx));
  $("fileListContainer").querySelectorAll("input[type=checkbox]").forEach(cb => cb.checked = true);
  $("count").textContent = selectedFiles.size;
};

$("deselectAll").onclick = () => {
  selectedFiles.clear();
  $("fileListContainer").querySelectorAll("input[type=checkbox]").forEach(cb => cb.checked = false);
  $("count").textContent = 0;
};

function selectedCrates() {
  const out = {};
  cratesBox.querySelectorAll("input").forEach((i) => { if (i.value.trim()) out[i.dataset.py] = i.value.trim(); });
  return out;
}

cratesBox.onclick = (e) => {
  const py = e.target.dataset.doc;
  if (!py) return;
  $("docText").value = docsMap[py] || "";
  $("docDialog").dataset.py = py;
  $("docDialog").showModal();
};
$("docDialog").onclose = () => {
  const py = $("docDialog").dataset.py;
  if (py) docsMap[py] = $("docText").value;
};

function askUser(question) {
  return new Promise((res) => {
    $("q").textContent = question; $("a").value = "";
    $("ask").showModal();
    $("ask").onclose = () => res($("a").value);
  });
}
const log = (s) => ($("log").textContent += s + "\n");

let isRunning = false;
let shouldStop = false;

function updateProgress(current, total) {
  const percent = Math.round((current / total) * 100);
  $("progressFill").style.width = percent + "%";
  $("progressText").textContent = `${current} / ${total} (${percent}%)`;
}

$("settingsBtn").onclick = () => {
  $("threadCount").value = localStorage.getItem("threadCount") || "4";
  $("webhookUrl").value = localStorage.getItem("webhookUrl") || "";
  $("darkTheme").checked = localStorage.getItem("darkTheme") === "true";
  $("settingsDialog").showModal();
};

$("settingsSave").onclick = () => {
  const threads = parseInt($("threadCount").value, 10);
  const dark = $("darkTheme").checked;
  const webhookUrl = $("webhookUrl").value.trim();
  localStorage.setItem("threadCount", threads);
  localStorage.setItem("darkTheme", dark);
  localStorage.setItem("webhookUrl", webhookUrl);
  document.body.classList.toggle("dark", dark);
  $("settingsDialog").close();
  log(`\n✓ ${T.settingsSaved || "Settings saved"}`);
};

// Load saved settings
const savedThreads = localStorage.getItem("threadCount");
if (savedThreads) $("threadCount").value = savedThreads;
const savedDark = localStorage.getItem("darkTheme") === "true";
$("darkTheme").checked = savedDark;
document.body.classList.toggle("dark", savedDark);

// Profile management
$("saveProfile").onclick = () => $("saveProfileDialog").showModal();

$("profileCancelBtn").onclick = () => $("saveProfileDialog").close();

$("profileSaveBtn").onclick = () => {
  const name = $("profileNameInput").value.trim();
  if (!name) {
    alert(T.profileNameRequired || "Profile name is required");
    return;
  }

  const profile = {
    name,
    kind: $("kind").value,
    base_url: $("base").value,
    model: $("model").value,
    api_key: $("key").value || null,
    custom_body: $("customBody")?.value || null,
    custom_response_path: $("customResponsePath")?.value || null,
    custom_headers: $("customHeaders")?.value || null
  };

  const profiles = JSON.parse(localStorage.getItem("profiles") || "[]");
  const existingIdx = profiles.findIndex(p => p.name === name);
  if (existingIdx >= 0) {
    profiles[existingIdx] = profile;
  } else {
    profiles.push(profile);
  }
  localStorage.setItem("profiles", JSON.stringify(profiles));

  $("profileNameInput").value = "";
  $("saveProfileDialog").close();
  log(`\n✓ ${T.profileSaved || "Profile saved"}: ${name}`);
};

$("loadProfile").onclick = () => {
  const profiles = JSON.parse(localStorage.getItem("profiles") || "[]");
  const list = $("profileList");
  list.innerHTML = "";

  if (profiles.length === 0) {
    list.innerHTML = `<p style="opacity:0.6">${T.noProfiles || "No saved profiles"}</p>`;
  } else {
    profiles.forEach((profile, idx) => {
      const div = document.createElement("div");
      div.style.display = "flex";
      div.style.gap = "8px";
      div.style.padding = "8px";
      div.style.border = "1px solid var(--line)";
      div.style.borderRadius = "6px";
      div.innerHTML = `
        <div style="flex:1">
          <strong>${profile.name}</strong><br>
          <small style="opacity:0.7">${profile.kind} - ${profile.model}</small>
        </div>
        <button type="button" data-load="${idx}" class="primary" style="padding:6px 12px">${T.profileLoad || "Load"}</button>
        <button type="button" data-delete="${idx}" style="padding:6px 12px">🗑️</button>
      `;
      list.appendChild(div);
    });
  }

  $("profileDialog").showModal();
};

$("profileList").onclick = (e) => {
  const profiles = JSON.parse(localStorage.getItem("profiles") || "[]");

  if (e.target.dataset.load !== undefined) {
    const profile = profiles[parseInt(e.target.dataset.load, 10)];
    $("kind").value = profile.kind;
    $("base").value = profile.base_url;
    $("model").value = profile.model;
    $("key").value = profile.api_key || "";
    if ($("customBody")) $("customBody").value = profile.custom_body || "";
    if ($("customResponsePath")) $("customResponsePath").value = profile.custom_response_path || "";
    if ($("customHeaders")) $("customHeaders").value = profile.custom_headers || "";
    $("kind").dispatchEvent(new Event("change"));
    $("profileDialog").close();
    log(`\n✓ ${T.profileLoaded || "Profile loaded"}: ${profile.name}`);
  }

  if (e.target.dataset.delete !== undefined) {
    const idx = parseInt(e.target.dataset.delete, 10);
    const name = profiles[idx].name;
    if (confirm(`${T.profileDeleteConfirm || "Delete profile"} "${name}"?`)) {
      profiles.splice(idx, 1);
      localStorage.setItem("profiles", JSON.stringify(profiles));
      $("loadProfile").click();
      log(`\n✓ ${T.profileDeleted || "Profile deleted"}: ${name}`);
    }
  }
};

$("profileClose").onclick = () => $("profileDialog").close();

// Migration history
function saveToHistory(results, startTime, endTime, metrics) {
  const history = JSON.parse(localStorage.getItem("migrationHistory") || "[]");
  const totalFiles = results.length;
  const successFiles = results.filter(r => r.checked).length;
  const duration = Math.round((endTime - startTime) / 1000);

  const entry = {
    timestamp: new Date().toISOString(),
    project: root,
    totalFiles,
    successFiles,
    failedFiles: totalFiles - successFiles,
    duration,
    metrics: metrics || {}, // Performance metrics
    results
  };

  history.unshift(entry);
  if (history.length > 50) history.pop(); // Keep last 50 migrations
  localStorage.setItem("migrationHistory", JSON.stringify(history));
}

$("historyBtn").onclick = () => {
  const history = JSON.parse(localStorage.getItem("migrationHistory") || "[]");
  const list = $("historyList");
  list.innerHTML = "";

  if (history.length === 0) {
    list.innerHTML = `<p style="opacity:0.6">${T.noHistory || "No migration history"}</p>`;
  } else {
    history.forEach((entry, idx) => {
      const date = new Date(entry.timestamp);
      const dateStr = date.toLocaleString();
      const successRate = Math.round((entry.successFiles / entry.totalFiles) * 100);

      const div = document.createElement("div");
      div.style.border = "1px solid var(--line)";
      div.style.borderRadius = "6px";
      div.style.padding = "12px";
      div.style.marginBottom = "8px";

      // Build metrics display
      let metricsHtml = '';
      if (entry.metrics) {
        const m = entry.metrics;
        metricsHtml = '<div style="font-size:12px;opacity:0.7;margin-top:4px">';
        if (m.avgFileTime) metricsHtml += `⏱ Avg: ${m.avgFileTime.toFixed(1)}s/file · `;
        if (m.apiCalls) metricsHtml += `📡 ${m.apiCalls} API calls · `;
        if (m.totalTokens) metricsHtml += `🔢 ${m.totalTokens.toLocaleString()} tokens · `;
        if (m.totalCost) metricsHtml += `💰 $${m.totalCost.toFixed(4)}`;
        metricsHtml += '</div>';
      }

      div.innerHTML = `
        <div style="display:flex;justify-content:space-between;margin-bottom:8px">
          <strong>${dateStr}</strong>
          <span style="color:${successRate === 100 ? 'green' : successRate > 50 ? 'orange' : 'red'}">${successRate}%</span>
        </div>
        <div style="font-size:14px;opacity:0.8">
          <div>📁 ${entry.project}</div>
          <div>✓ ${entry.successFiles} / ${entry.totalFiles} files · ⏱ ${entry.duration}s</div>
          ${metricsHtml}
        </div>
        <div style="margin-top:8px;display:flex;gap:8px">
          <button type="button" data-view="${idx}" style="font-size:13px;padding:4px 10px">${T.viewReport || "View Report"}</button>
          <button type="button" data-delete-history="${idx}" style="font-size:13px;padding:4px 10px">🗑️</button>
        </div>
      `;
      list.appendChild(div);
    });
  }

  $("historyDialog").showModal();
};

$("historyList").onclick = (e) => {
  const history = JSON.parse(localStorage.getItem("migrationHistory") || "[]");

  if (e.target.dataset.view !== undefined) {
    const entry = history[parseInt(e.target.dataset.view, 10)];

    // Build detailed report with metrics
    let report = `=== Migration Report ===\nDate: ${new Date(entry.timestamp).toLocaleString()}\nProject: ${entry.project}\nSuccess: ${entry.successFiles}/${entry.totalFiles}\nDuration: ${entry.duration}s\n`;

    if (entry.metrics) {
      const m = entry.metrics;
      report += '\n--- Performance Metrics ---\n';
      if (m.avgFileTime) report += `Average file time: ${m.avgFileTime.toFixed(1)}s\n`;
      if (m.apiCalls) report += `API calls: ${m.apiCalls}`;
      if (m.retries) report += ` (${m.retries} retries)`;
      report += '\n';
      if (m.totalTokens) report += `Total tokens: ${m.totalTokens.toLocaleString()}\n`;
      if (m.totalCost) report += `Estimated cost: $${m.totalCost.toFixed(4)}\n`;
    }

    const reportText = entry.results.map(r =>
      `${r.checked ? '✓' : '✗'} ${r.file}${r.time ? ` (${(r.time/1000).toFixed(1)}s)` : ''}${r.errors ? '\n  Error: ' + r.errors : ''}`
    ).join('\n');

    $("log").textContent = report + `\n${reportText}`;
    $("historyDialog").close();
  }

  if (e.target.dataset.deleteHistory !== undefined) {
    if (confirm(T.deleteHistoryConfirm || "Delete this history entry?")) {
      history.splice(parseInt(e.target.dataset.deleteHistory, 10), 1);
      localStorage.setItem("migrationHistory", JSON.stringify(history));
      $("historyBtn").click();
    }
  }
};

$("historyClose").onclick = () => $("historyDialog").close();

$("stop").onclick = () => {
  shouldStop = true;
  $("stop").disabled = true;
  log("\n⏹ Остановка...");
};

$("start").onclick = async () => {
  shouldStop = false;
  const startTime = Date.now();
  const customBody = $("customBody")?.value || null;
  const customResponsePath = $("customResponsePath")?.value || null;
  const customHeaders = $("customHeaders")?.value || null;

  const qualityLevel = $("qualityLevel").value;
  const generateTests = $("generateTests").checked;
  const generateReadme = $("generateReadme").checked;

  const provider = {
    kind: $("kind").value,
    base_url: $("base").value,
    api_key: $("key").value || null,
    model: $("model").value,
    custom_body: customBody,
    custom_response_path: customResponsePath,
    custom_headers: customHeaders
  };
  const results = [];
  const crates = selectedCrates();
  const docs = { ...docsMap };

  // Initialize performance metrics
  const metrics = {
    totalTokens: 0,
    totalCost: 0,
    avgFileTime: 0,
    fileTimes: [],
    apiCalls: 0,
    retries: 0,
  };

  $("start").disabled = true;
  $("start").style.display = "none";
  $("stop").style.display = "inline-block";
  $("stop").disabled = false;
  $("log").textContent = "";
  $("progress").style.display = "block";
  updateProgress(0, files.length);

  for (let idx = 0; idx < files.length; idx++) {
    if (shouldStop) {
      log(`\n⏹ Миграция остановлена пользователем на файле ${idx}/${files.length}`);
      break;
    }

    // Skip unselected files
    if (!selectedFiles.has(idx)) {
      continue;
    }

    const { file, deps } = files[idx];
    const progress = `[${idx + 1}/${files.length}]`;
    log(`${progress} ${file}...`);
    updateProgress(idx, files.length);

    const fileStartTime = Date.now();
    const threadCount = parseInt(localStorage.getItem("threadCount") || "4", 10);
    const answers = [];
    for (let i = 0; i < 5; i++) {
      try {
        const r = await invoke("migrate_file", {
          a: {
            root,
            file,
            description: $("desc").value,
            deps,
            crates,
            docs,
            lang: $("lang").value,
            answers,
            provider,
            thread_count: threadCount,
            quality_level: qualityLevel,
            generate_tests: generateTests
          }
        });
        if (r.kind === "Ask") { answers.push(await askUser(r.question)); continue; }

        const fileTime = Date.now() - fileStartTime;
        metrics.fileTimes.push(fileTime);
        metrics.apiCalls++;
        if (i > 0) metrics.retries += i;

        // Estimate tokens and cost (rough approximation)
        if (r.tokens) {
          metrics.totalTokens += r.tokens;
          // Rough cost estimates (varies by provider/model)
          const costPerToken = getCostPerToken(provider.kind, provider.model);
          metrics.totalCost += r.tokens * costPerToken;
        }

        log(`${progress} ${r.checked ? T.checkOk : T.checkFail}: ${file} (${(fileTime/1000).toFixed(1)}s)`);
        if (!r.checked && r.errors) log(`  ${r.errors}`);
        results.push({ file, out_path: r.out_path, checked: r.checked, errors: r.errors, time: fileTime });
        break;
      } catch (e) {
        log(`${progress} ${T.error}: ${file}: ${e}`);
        results.push({ file, out_path: "", checked: false, errors: String(e), time: Date.now() - fileStartTime });
        break;
      }
    }
  }

  updateProgress(files.length, files.length);
  const endTime = Date.now();
  const totalDuration = (endTime - startTime) / 1000;

  // Calculate average file time
  if (metrics.fileTimes.length > 0) {
    metrics.avgFileTime = metrics.fileTimes.reduce((a, b) => a + b, 0) / metrics.fileTimes.length / 1000;
  }

  if (!shouldStop && results.length > 0) {
    const [ok, reportPath] = await invoke("finalize", { root, results });
    log(`\n${ok ? T.projectOk : T.projectFail}`);
    log(`Report: ${reportPath}`);

    // Generate README.md for Rust project if requested
    if (generateReadme) {
      try {
        log(`\nGenerating README.md for Rust project...`);
        await invoke("generate_readme", {
          root,
          project_description: $("desc").value,
          crates,
          results,
          provider
        });
        log(`✓ README.md created`);
      } catch (e) {
        log(`⚠ Failed to generate README: ${e}`);
      }
    }

    // Log performance metrics
    log(`\n--- Performance Metrics ---`);
    log(`Total duration: ${totalDuration.toFixed(1)}s`);
    log(`Average file time: ${metrics.avgFileTime.toFixed(1)}s`);
    log(`API calls: ${metrics.apiCalls} (${metrics.retries} retries)`);
    if (metrics.totalTokens > 0) {
      log(`Total tokens: ${metrics.totalTokens.toLocaleString()}`);
      log(`Estimated cost: $${metrics.totalCost.toFixed(4)}`);
    }

    // Automatically open report
    const reportContent = results.map(r =>
      `${r.checked ? '✓' : '✗'} ${r.file} → ${r.out_path}${r.errors ? '\n  Error: ' + r.errors : ''}`
    ).join('\n');
    log(`\n--- Migration Summary ---\n${reportContent}`);

    // Save to history with metrics
    saveToHistory(results, startTime, endTime, metrics);

    // Show export button
    $("exportReport").style.display = "inline-block";
    $("exportReport").dataset.results = JSON.stringify({results, startTime, endTime, metrics, root});

    // Send webhook notification if configured
    const webhookUrl = localStorage.getItem("webhookUrl");
    if (webhookUrl) {
      sendWebhookNotification(webhookUrl, {
        project: root,
        totalFiles,
        successFiles: results.filter(r => r.checked).length,
        failedFiles: results.filter(r => !r.checked).length,
        duration: totalDuration,
        metrics,
        timestamp: new Date().toISOString()
      });
    }
  }

  $("start").disabled = false;
  $("start").style.display = "inline-block";
  $("stop").style.display = "none";
};

// Webhook notification
async function sendWebhookNotification(url, data) {
  try {
    await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data)
    });
    log(`\n✓ Webhook notification sent to ${url}`);
  } catch (e) {
    log(`\n⚠ Failed to send webhook: ${e}`);
  }
}

// Export functionality
$("exportReport").onclick = () => {
  $("exportDialog").showModal();
};

$("exportJSON").onclick = () => {
  const data = JSON.parse($("exportReport").dataset.results);
  const json = JSON.stringify({
    project: data.root,
    timestamp: new Date().toISOString(),
    duration: (data.endTime - data.startTime) / 1000,
    metrics: data.metrics,
    results: data.results
  }, null, 2);

  const blob = new Blob([json], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `py2rs-report-${Date.now()}.json`;
  a.click();
  URL.revokeObjectURL(url);
  $("exportDialog").close();
  log(`\n✓ Report exported as JSON`);
};

$("exportCSV").onclick = () => {
  const data = JSON.parse($("exportReport").dataset.results);
  const rows = [["File", "Output", "Status", "Time(s)", "Errors"]];
  data.results.forEach(r => {
    rows.push([
      r.file,
      r.out_path || "",
      r.checked ? "Success" : "Failed",
      r.time ? (r.time / 1000).toFixed(2) : "",
      r.errors || ""
    ]);
  });

  const csv = rows.map(row => row.map(cell => `"${String(cell).replace(/"/g, '""')}"`).join(",")).join("\n");
  const blob = new Blob([csv], { type: "text/csv" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `py2rs-report-${Date.now()}.csv`;
  a.click();
  URL.revokeObjectURL(url);
  $("exportDialog").close();
  log(`\n✓ Report exported as CSV`);
};

$("exportMarkdown").onclick = () => {
  const data = JSON.parse($("exportReport").dataset.results);
  const successCount = data.results.filter(r => r.checked).length;
  const failCount = data.results.filter(r => !r.checked).length;
  const duration = (data.endTime - data.startTime) / 1000;

  let md = `# py2rs Migration Report\n\n`;
  md += `**Project:** ${data.root}\n`;
  md += `**Date:** ${new Date().toLocaleString()}\n`;
  md += `**Duration:** ${duration.toFixed(1)}s\n`;
  md += `**Success:** ${successCount}/${data.results.length}\n\n`;

  if (data.metrics) {
    md += `## Performance Metrics\n\n`;
    if (data.metrics.avgFileTime) md += `- Average file time: ${data.metrics.avgFileTime.toFixed(1)}s\n`;
    if (data.metrics.apiCalls) md += `- API calls: ${data.metrics.apiCalls} (${data.metrics.retries || 0} retries)\n`;
    if (data.metrics.totalTokens) md += `- Total tokens: ${data.metrics.totalTokens.toLocaleString()}\n`;
    if (data.metrics.totalCost) md += `- Estimated cost: $${data.metrics.totalCost.toFixed(4)}\n`;
    md += `\n`;
  }

  md += `## Files\n\n`;
  md += `| File | Status | Time | Output |\n`;
  md += `|------|--------|------|--------|\n`;
  data.results.forEach(r => {
    const status = r.checked ? "✓" : "✗";
    const time = r.time ? `${(r.time/1000).toFixed(1)}s` : "-";
    md += `| ${r.file} | ${status} | ${time} | ${r.out_path || "-"} |\n`;
  });

  if (failCount > 0) {
    md += `\n## Errors\n\n`;
    data.results.filter(r => !r.checked && r.errors).forEach(r => {
      md += `### ${r.file}\n\n\`\`\`\n${r.errors}\n\`\`\`\n\n`;
    });
  }

  const blob = new Blob([md], { type: "text/markdown" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `py2rs-report-${Date.now()}.md`;
  a.click();
  URL.revokeObjectURL(url);
  $("exportDialog").close();
  log(`\n✓ Report exported as Markdown`);
};

$("exportClose").onclick = () => $("exportDialog").close();

// Helper function to estimate cost per token
function getCostPerToken(kind, model) {
  // Rough estimates in USD per token (input + output average)
  // These are approximations and should be updated based on actual pricing
  const costs = {
    'openaicompatible': {
      'gpt-4': 0.00003,
      'gpt-4-turbo': 0.00001,
      'gpt-3.5-turbo': 0.000001,
      'default': 0.00001
    },
    'anthropic': {
      'claude-opus-4': 0.00001,
      'claude-sonnet': 0.000003,
      'default': 0.000003
    },
    'openrouter': {
      'default': 0.00001
    },
    'groq': {
      'default': 0.0000002 // Much cheaper
    },
    'llamacpp': {
      'default': 0 // Local
    },
    'default': 0.000001
  };

  if (costs[kind]) {
    const modelCosts = costs[kind];
    for (const modelKey in modelCosts) {
      if (model && model.includes(modelKey)) {
        return modelCosts[modelKey];
      }
    }
    return modelCosts['default'] || costs['default'];
  }
  return costs['default'];
}
