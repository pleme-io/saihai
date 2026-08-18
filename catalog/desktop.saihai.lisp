;; ─────────────────────────────────────────────────────────────────────
;; THE DESKTOP ACTION CATALOG — the source every surface is generated from.
;;
;; One (defaction …) row per thing a pleme-io desktop can be asked to do.
;; From these rows come the Rust action enum and dispatch, the SDKs, their
;; conformance tests, and the reconciler's plan vocabulary. Nothing downstream
;; may invent a verb.
;;
;; ── THE ONE RULE AN AUTHOR MUST UNDERSTAND ──────────────────────────
;; `:observed` is not documentation. It is the ONLY statement of whether an
;; action's effect can be read back, and the class follows from it:
;;
;;   :kind :observe                  -> Read        (never planned)
;;   :kind :mutate + observed rows   -> Converging  (reconciler may converge)
;;   :kind :mutate + observed EMPTY  -> Blind       (may be fired, NEVER counted)
;;
;; There is deliberately no `:observability` field. An author names what can
;; be read; nobody gets to assert convergeability.
;;
;; ── AUTHORITY ────────────────────────────────────────────────────────
;;   :l0  unprivileged — own process, public reads
;;   :l1  seat-owner — connected to the seat
;;   :l2  controller-privileged — controller protocols, capture, output mgmt
;;   :l3  break-glass — needs a Warrant; no automated loop may mint one
;; ─────────────────────────────────────────────────────────────────────

;; ── WINDOW ───────────────────────────────────────────────────────────

(defaction :id "window-focus" :gloss "Give a window keyboard focus"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ((defobserved :domain :focus :field "window")))

(defaction :id "window-focus-direction" :gloss "Move focus spatially"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "direction" :kind :direction :required #t)
           (defparam :name "wrap" :kind :bool :required #f))
  :observed ((defobserved :domain :focus :field "window")))

(defaction :id "window-move" :gloss "Move a window to a position"
  :category :window :kind :mutate :auth :l2
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "x" :kind :int :required #t)
           (defparam :name "y" :kind :int :required #t))
  :observed ((defobserved :domain :windows :field "position")))

(defaction :id "window-resize" :gloss "Resize a window"
  :category :window :kind :mutate :auth :l2
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "width" :kind :int :required #t)
           (defparam :name "height" :kind :int :required #t))
  :observed ((defobserved :domain :windows :field "size")))

(defaction :id "window-close" :gloss "Ask a window to close"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ((defobserved :domain :windows :field "present")))

(defaction :id "window-set-fullscreen" :gloss "Set or clear fullscreen"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :windows :field "fullscreen")))

(defaction :id "window-set-maximized" :gloss "Set or clear maximized"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :windows :field "maximized")))

(defaction :id "window-set-floating" :gloss "Float or tile a window"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :windows :field "floating")))

;; BLIND: stacking order is exposed by no Wayland protocol and no compositor
;; IPC read. It may be commanded and can never be confirmed, so the reconciler
;; must never count it toward a fixpoint. That is what an empty :observed says.
(defaction :id "window-raise" :gloss "Raise a window in the stacking order"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ())

(defaction :id "window-lower" :gloss "Lower a window in the stacking order"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ())

(defaction :id "window-to-workspace" :gloss "Send a window to a workspace"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "workspace" :kind :selector :required #t)
           (defparam :name "follow" :kind :bool :required #f))
  :observed ((defobserved :domain :windows :field "workspace")))

(defaction :id "window-to-output" :gloss "Send a window to another output"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "output" :kind :selector :required #t))
  :observed ((defobserved :domain :windows :field "output")))

(defaction :id "window-swap" :gloss "Swap two windows in the layout"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "other" :kind :selector :required #t))
  :observed ((defobserved :domain :layout :field "order")))

(defaction :id "window-list" :gloss "List managed windows"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "filter" :kind :str :required #f))
  :observed ((defobserved :domain :windows :field "all")))

(defaction :id "window-get" :gloss "Read one window's full state"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ((defobserved :domain :windows :field "one")))

;; ── WORKSPACE ────────────────────────────────────────────────────────

(defaction :id "workspace-focus" :gloss "Switch to a workspace"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #t))
  :observed ((defobserved :domain :focus :field "workspace")))

(defaction :id "workspace-cycle" :gloss "Move to the next or previous workspace"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "direction" :kind :direction :required #t)
           (defparam :name "wrap" :kind :bool :required #f))
  :observed ((defobserved :domain :focus :field "workspace")))

(defaction :id "workspace-create" :gloss "Create a workspace"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "name" :kind :str :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :workspaces :field "present")))

(defaction :id "workspace-destroy" :gloss "Destroy a workspace"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #t))
  :observed ((defobserved :domain :workspaces :field "present")))

(defaction :id "workspace-rename" :gloss "Rename a workspace"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #t)
           (defparam :name "name" :kind :str :required #t))
  :observed ((defobserved :domain :workspaces :field "name")))

(defaction :id "workspace-move-to-output" :gloss "Move a workspace to an output"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #t)
           (defparam :name "output" :kind :selector :required #t))
  :observed ((defobserved :domain :workspaces :field "output")))

(defaction :id "workspace-list" :gloss "List workspaces"
  :category :workspace :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :workspaces :field "all")))

;; ── LAYOUT ───────────────────────────────────────────────────────────

(defaction :id "layout-set" :gloss "Set a workspace's layout"
  :category :layout :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #t)
           (defparam :name "layout" :kind :str :required #t))
  :observed ((defobserved :domain :layout :field "kind")))

(defaction :id "layout-set-orientation" :gloss "Set the split orientation"
  :category :layout :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #t)
           (defparam :name "orientation" :kind :str :required #t))
  :observed ((defobserved :domain :layout :field "orientation")))

(defaction :id "layout-set-gaps" :gloss "Set inner and outer gaps"
  :category :layout :kind :mutate :auth :l1
  :params ((defparam :name "inner" :kind :int :required #t)
           (defparam :name "outer" :kind :int :required #t))
  :observed ((defobserved :domain :layout :field "gaps")))

(defaction :id "layout-equalize" :gloss "Equalize sibling sizes"
  :category :layout :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #f))
  :observed ((defobserved :domain :layout :field "sizes")))

(defaction :id "layout-get" :gloss "Read a workspace's layout"
  :category :layout :kind :observe :auth :l0
  :params ((defparam :name "workspace" :kind :selector :required #t))
  :observed ((defobserved :domain :layout :field "all")))

;; ── OUTPUT ───────────────────────────────────────────────────────────

(defaction :id "output-set-enabled" :gloss "Enable or disable an output"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "output" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :outputs :field "enabled")))

(defaction :id "output-set-mode" :gloss "Set an output's resolution and refresh"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "output" :kind :selector :required #t)
           (defparam :name "width" :kind :int :required #t)
           (defparam :name "height" :kind :int :required #t)
           (defparam :name "refresh" :kind :int :required #f))
  :observed ((defobserved :domain :outputs :field "mode")))

(defaction :id "output-set-scale" :gloss "Set an output's scale factor"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "output" :kind :selector :required #t)
           (defparam :name "scale" :kind :int :required #t))
  :observed ((defobserved :domain :outputs :field "scale")))

(defaction :id "output-set-transform" :gloss "Rotate or flip an output"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "output" :kind :selector :required #t)
           (defparam :name "transform" :kind :str :required #t))
  :observed ((defobserved :domain :outputs :field "transform")))

(defaction :id "output-set-position" :gloss "Place an output in the layout"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "output" :kind :selector :required #t)
           (defparam :name "x" :kind :int :required #t)
           (defparam :name "y" :kind :int :required #t))
  :observed ((defobserved :domain :outputs :field "position")))

(defaction :id "output-list" :gloss "List outputs and their modes"
  :category :output :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :outputs :field "all")))

;; ── INPUT / KEYBOARD CONTROL ─────────────────────────────────────────

(defaction :id "input-set-repeat" :gloss "Set key repeat rate and delay"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "rate" :kind :int :required #t)
           (defparam :name "delay" :kind :int :required #t))
  :observed ((defobserved :domain :inputs :field "repeat")))

(defaction :id "input-set-layout" :gloss "Set the keyboard layout"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "layout" :kind :str :required #t)
           (defparam :name "variant" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "layout")))

(defaction :id "input-bind" :gloss "Bind a key chord to an action"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "chord" :kind :str :required #t)
           (defparam :name "action" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "bindings")))

(defaction :id "input-unbind" :gloss "Remove a key binding"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "chord" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "bindings")))

(defaction :id "input-set-mode" :gloss "Switch the active keymap mode"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "mode" :kind :str :required #t))
  :observed ((defobserved :domain :inputs :field "mode")))

;; BLIND: synthetic input leaves no readable trace distinguishable from a real
;; keystroke, and it is break-glass for the same reason.
(defaction :id "input-send-keys" :gloss "Inject a synthetic key sequence"
  :category :input :kind :mutate :auth :l3
  :params ((defparam :name "keys" :kind :str :required #t)
           (defparam :name "target" :kind :selector :required #f))
  :observed ())

(defaction :id "input-list-bindings" :gloss "List active key bindings"
  :category :input :kind :observe :auth :l0
  :params ((defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "bindings")))

;; ── FOCUS POLICY ─────────────────────────────────────────────────────

(defaction :id "focus-set-policy" :gloss "Set the focus-follows-mouse policy"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "policy" :kind :str :required #t))
  :observed ((defobserved :domain :focus :field "policy")))

(defaction :id "focus-get" :gloss "Read what currently has focus"
  :category :focus :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :focus :field "all")))

;; ── SESSION ──────────────────────────────────────────────────────────

(defaction :id "session-lock" :gloss "Lock the session"
  :category :session :kind :mutate :auth :l1
  :params ()
  :observed ((defobserved :domain :session :field "locked")))

(defaction :id "session-unlock" :gloss "Unlock the session after authentication"
  :category :session :kind :mutate :auth :l1
  :params ()
  :observed ((defobserved :domain :session :field "locked")))

(defaction :id "session-logout" :gloss "End the session"
  :category :session :kind :mutate :auth :l1
  :params ()
  :observed ((defobserved :domain :session :field "active")))

(defaction :id "session-suspend" :gloss "Suspend the machine"
  :category :session :kind :mutate :auth :l2
  :params ()
  :observed ())

(defaction :id "session-reboot" :gloss "Reboot the machine"
  :category :session :kind :mutate :auth :l2
  :params ()
  :observed ())

(defaction :id "session-poweroff" :gloss "Power the machine off"
  :category :session :kind :mutate :auth :l2
  :params ()
  :observed ())

(defaction :id "session-switch-vt" :gloss "Switch to another virtual terminal"
  :category :session :kind :mutate :auth :l2
  :params ((defparam :name "vt" :kind :int :required #t))
  :observed ((defobserved :domain :session :field "vt")))

(defaction :id "session-get" :gloss "Read session state"
  :category :session :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "all")))

;; ── LOGIN ────────────────────────────────────────────────────────────
;;
;; The greeter half. These are the actions a login manager performs, and they
;; are in the SAME catalog as the desktop's on purpose: the greeter is a mode
;; of the compositor rather than a client of one, so a separate vocabulary
;; would be a second system to keep in step.

(defaction :id "login-authenticate" :gloss "Authenticate a user against PAM"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "user" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "authenticated")))

(defaction :id "login-start-session" :gloss "Start a session for an authenticated user"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "user" :kind :str :required #t)
           (defparam :name "session" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "active")))

(defaction :id "login-end-session" :gloss "End a session and close its PAM handle"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "user" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "active")))

(defaction :id "login-switch-user" :gloss "Switch to another user's session"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "user" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "user")))

(defaction :id "login-list-sessions" :gloss "List active sessions"
  :category :login :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "sessions")))

;; ── SEAT ─────────────────────────────────────────────────────────────

(defaction :id "seat-get" :gloss "Read the seat's identity and devices"
  :category :seat :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "seat")))

;; ── LAUNCH ───────────────────────────────────────────────────────────

(defaction :id "launch-spawn" :gloss "Spawn a program"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "command" :kind :str :required #t)
           (defparam :name "workspace" :kind :selector :required #f))
  :observed ((defobserved :domain :windows :field "present")))

(defaction :id "launch-focus-or-spawn" :gloss "Focus a program's window, or spawn it"
  :category :launch :kind :mutate :auth :l1
  ;; `criteria`, not `match` — the border rejects `match` as reserved (Rust),
  ;; which is the guard working: one catalog error beats three emitter bugs.
  :params ((defparam :name "criteria" :kind :selector :required #t)
           (defparam :name "command" :kind :str :required #t))
  :observed ((defobserved :domain :focus :field "window")))

;; ── CLIPBOARD ────────────────────────────────────────────────────────

(defaction :id "clipboard-get" :gloss "Read the clipboard"
  :category :clipboard :kind :observe :auth :l1
  :params ((defparam :name "selection" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "clipboard")))

(defaction :id "clipboard-put" :gloss "Write the clipboard"
  :category :clipboard :kind :mutate :auth :l1
  :params ((defparam :name "content" :kind :str :required #t)
           (defparam :name "selection" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "clipboard")))

(defaction :id "clipboard-clear" :gloss "Clear the clipboard"
  :category :clipboard :kind :mutate :auth :l1
  :params ((defparam :name "selection" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "clipboard")))

;; ── CAPTURE ──────────────────────────────────────────────────────────
;;
;; BLIND by nature: a screenshot's effect is a file, not a state a desktop
;; reader reports. Firing it twice makes two files, which is exactly why it
;; must never be converged on.

(defaction :id "capture-screenshot" :gloss "Capture a screenshot"
  :category :capture :kind :mutate :auth :l2
  :params ((defparam :name "target" :kind :selector :required #f)
           (defparam :name "path" :kind :str :required #f))
  :observed ())

(defaction :id "capture-screencast-start" :gloss "Start a screencast"
  :category :capture :kind :mutate :auth :l2
  :params ((defparam :name "target" :kind :selector :required #f))
  :observed ())

(defaction :id "capture-screencast-stop" :gloss "Stop a screencast"
  :category :capture :kind :mutate :auth :l2
  :params ()
  :observed ())

;; ── NOTIFY ───────────────────────────────────────────────────────────

(defaction :id "notify-send" :gloss "Post a notification"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "summary" :kind :str :required #t)
           (defparam :name "body" :kind :str :required #f)
           (defparam :name "urgency" :kind :str :required #f))
  :observed ())

(defaction :id "notify-dismiss" :gloss "Dismiss a notification"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ())

;; ── THEME ────────────────────────────────────────────────────────────
;;
;; The seam to pente: the desktop selects a theme, pente renders it.

(defaction :id "theme-select" :gloss "Select the active theme"
  :category :theme :kind :mutate :auth :l1
  :params ((defparam :name "name" :kind :str :required #t))
  :observed ((defobserved :domain :theme :field "name")))

(defaction :id "theme-reload" :gloss "Re-read the active theme's artifacts"
  :category :theme :kind :mutate :auth :l1
  :params ()
  :observed ((defobserved :domain :theme :field "revision")))

(defaction :id "theme-get" :gloss "Read the active theme"
  :category :theme :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :theme :field "all")))

;; ── SYSTEM ───────────────────────────────────────────────────────────

(defaction :id "system-reload-config" :gloss "Re-read the desktop configuration"
  :category :system :kind :mutate :auth :l1
  :params ()
  :observed ((defobserved :domain :session :field "revision")))

(defaction :id "system-get-state" :gloss "Read the whole observable desktop state"
  :category :system :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "all")
             (defobserved :domain :workspaces :field "all")
             (defobserved :domain :outputs :field "all")
             (defobserved :domain :focus :field "all")
             (defobserved :domain :session :field "all")))

(defaction :id "system-version" :gloss "Read the desktop's version and backend"
  :category :system :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "version")))
