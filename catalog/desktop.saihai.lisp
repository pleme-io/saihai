;; ─────────────────────────────────────────────────────────────────────
;; THE DESKTOP ACTION CATALOG — 248 actions.
;;
;; GENERATED from docs/saihai-desktop-action-spec.md §3 by
;; scratchpad/convert_catalog.py, then committed. Regenerating is how new rows
;; land; hand-editing one row is fine, hand-editing many means the spec and the
;; catalog have diverged.
;;
;; ── WHAT IS FAITHFUL, AND WHAT IS DERIVED ───────────────────────────
;; Faithful from the spec, because these are the load-bearing axes:
;;   id, category, kind (mutate/observe), authority rung, and OBSERVABILITY —
;;   the spec's Obs/Blind column decides whether :observed is populated, and
;;   an empty :observed IS the statement that an action is blind.
;;
;; Derived, and said out loud rather than implied:
;;   :gloss              title-cased from the id; the spec's tables carry no
;;                       gloss column.
;;   :observed field     domain from the category, field from the action's
;;                       object. The CLASS is faithful; the specific field name
;;                       is a routing detail, refined per-action as real
;;                       backends land. A wrong field name shows up as a
;;                       Gap::NoAction — visible — not as silent success.
;;   :required           everything outside a known-optional set, ordered
;;                       required-first because the border rejects a required
;;                       parameter after an optional one (Go and positional
;;                       Python cannot express it).
;;
;; ── THE ONE RULE AN AUTHOR MUST UNDERSTAND ──────────────────────────
;;   :kind :observe                  -> Read        (never planned)
;;   :kind :mutate + observed rows   -> Converging  (reconciler may converge)
;;   :kind :mutate + observed EMPTY  -> Blind       (may fire, NEVER counted)
;;
;; There is deliberately no :observability field. An author names what can be
;; read back; nobody gets to assert convergeability.
;;
;; ── AUTHORITY ────────────────────────────────────────────────────────
;;   :l0 unprivileged · :l1 seat-owner · :l2 controller · :l3 break-glass
;; ─────────────────────────────────────────────────────────────────────

(defaction :id "window-focus" :gloss "Window focus"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ((defobserved :domain :focus :field "window")))

(defaction :id "window-focus-direction" :gloss "Window focus direction"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "direction" :kind :direction :required #t)
           (defparam :name "wrap" :kind :bool :required #f))
  :observed ((defobserved :domain :windows :field "direction")))

(defaction :id "window-move" :gloss "Window move"
  :category :window :kind :mutate :auth :l2
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "position" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "move")))

(defaction :id "window-move-direction" :gloss "Window move direction"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "direction" :kind :direction :required #t))
  :observed ((defobserved :domain :windows :field "direction")))

(defaction :id "window-resize" :gloss "Window resize"
  :category :window :kind :mutate :auth :l2
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "size" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "resize")))

(defaction :id "window-resize-delta" :gloss "Window resize delta"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "axis" :kind :str :required #t)
           (defparam :name "amount" :kind :int :required #t))
  :observed ((defobserved :domain :windows :field "delta")))

(defaction :id "window-close" :gloss "Window close"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ((defobserved :domain :windows :field "close")))

(defaction :id "window-set-fullscreen" :gloss "Window set fullscreen"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :windows :field "fullscreen")))

(defaction :id "window-set-maximized" :gloss "Window set maximized"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :windows :field "maximized")))

(defaction :id "window-set-minimized" :gloss "Window set minimized"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :windows :field "minimized")))

(defaction :id "window-set-floating" :gloss "Window set floating"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :windows :field "floating")))

(defaction :id "window-swap" :gloss "Window swap"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "a" :kind :selector :required #t)
           (defparam :name "b" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "swap")))

(defaction :id "window-raise" :gloss "Window raise"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ())

(defaction :id "window-lower" :gloss "Window lower"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ())

(defaction :id "window-center" :gloss "Window center"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "scope" :kind :str :required #f))
  :observed ((defobserved :domain :windows :field "center")))

(defaction :id "window-full-width" :gloss "Window full width"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ((defobserved :domain :windows :field "width")))

(defaction :id "window-equalize" :gloss "Window equalize"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "scope" :kind :str :required #f))
  :observed ((defobserved :domain :windows :field "equalize")))

(defaction :id "window-set-managed" :gloss "Window set managed"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "managed" :kind :bool :required #t))
  :observed ((defobserved :domain :windows :field "managed")))

(defaction :id "window-set-stacked" :gloss "Window set stacked"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :windows :field "stacked")))

(defaction :id "window-set-always-on-top" :gloss "Window set always on top"
  :category :window :kind :mutate :auth :l3
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ())

(defaction :id "window-to-output" :gloss "Window to output"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :windows :field "output")))

(defaction :id "window-to-workspace" :gloss "Window to workspace"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "workspace" :kind :selector :required #f)
           (defparam :name "follow" :kind :bool :required #f))
  :observed ((defobserved :domain :windows :field "workspace")))

(defaction :id "window-consume-or-expel" :gloss "Window consume or expel"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "direction" :kind :direction :required #t))
  :observed ((defobserved :domain :windows :field "expel")))

(defaction :id "window-list" :gloss "Window list"
  :category :window :kind :observe :auth :l1
  :params ((defparam :name "filter" :kind :str :required #f))
  :observed ((defobserved :domain :windows :field "list")))

(defaction :id "window-get" :gloss "Window get"
  :category :window :kind :observe :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ((defobserved :domain :windows :field "get")))

(defaction :id "window-pick" :gloss "Window pick"
  :category :window :kind :observe :auth :l1
  :params ((defparam :name "prompt" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "pick")))

(defaction :id "window-to-scratchpad" :gloss "Window to scratchpad"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "name" :kind :str :required #f))
  :observed ((defobserved :domain :windows :field "scratchpad")))

(defaction :id "scratchpad-show" :gloss "Scratchpad show"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "name" :kind :str :required #f))
  :observed ((defobserved :domain :windows :field "show")))

(defaction :id "scratchpad-hide" :gloss "Scratchpad hide"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "name" :kind :str :required #f))
  :observed ((defobserved :domain :windows :field "hide")))

(defaction :id "scratchpad-cycle" :gloss "Scratchpad cycle"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "direction" :kind :direction :required #t))
  :observed ((defobserved :domain :windows :field "cycle")))

(defaction :id "scratchpad-list" :gloss "Scratchpad list"
  :category :window :kind :observe :auth :l1
  :params ()
  :observed ((defobserved :domain :windows :field "list")))

(defaction :id "workspace-focus" :gloss "Workspace focus"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #f))
  :observed ((defobserved :domain :focus :field "workspace")))

(defaction :id "workspace-cycle" :gloss "Workspace cycle"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "direction" :kind :direction :required #t)
           (defparam :name "wrap" :kind :bool :required #f)
           (defparam :name "scope" :kind :str :required #f))
  :observed ((defobserved :domain :workspaces :field "cycle")))

(defaction :id "workspace-create" :gloss "Workspace create"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "name" :kind :str :required #f)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :workspaces :field "create")))

(defaction :id "workspace-destroy" :gloss "Workspace destroy"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #f)
           (defparam :name "on_nonempty" :kind :str :required #f))
  :observed ((defobserved :domain :workspaces :field "destroy")))

(defaction :id "workspace-rename" :gloss "Workspace rename"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #f)
           (defparam :name "name" :kind :str :required #f))
  :observed ((defobserved :domain :workspaces :field "rename")))

(defaction :id "workspace-move-to-output" :gloss "Workspace move to output"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #f)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :workspaces :field "output")))

(defaction :id "workspace-reorder" :gloss "Workspace reorder"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "direction" :kind :direction :required #t)
           (defparam :name "workspace" :kind :selector :required #f))
  :observed ((defobserved :domain :workspaces :field "reorder")))

(defaction :id "workspace-set-urgent" :gloss "Workspace set urgent"
  :category :workspace :kind :mutate :auth :l1
  :params ((defparam :name "urgent" :kind :str :required #t)
           (defparam :name "workspace" :kind :selector :required #f))
  :observed ((defobserved :domain :workspaces :field "urgent")))

(defaction :id "workspace-list" :gloss "Workspace list"
  :category :workspace :kind :observe :auth :l1
  :params ()
  :observed ((defobserved :domain :workspaces :field "list")))

(defaction :id "layout-set" :gloss "Layout set"
  :category :layout :kind :mutate :auth :l1
  :params ((defparam :name "layout" :kind :str :required #t)
           (defparam :name "accordion" :kind :str :required #t)
           (defparam :name "floating" :kind :str :required #t)
           (defparam :name "auto" :kind :str :required #t)
           (defparam :name "workspace" :kind :selector :required #f))
  :observed ((defobserved :domain :layout :field "set")))

(defaction :id "layout-set-orientation" :gloss "Layout set orientation"
  :category :layout :kind :mutate :auth :l1
  :params ((defparam :name "orientation" :kind :str :required #t)
           (defparam :name "workspace" :kind :selector :required #f))
  :observed ((defobserved :domain :layout :field "orientation")))

(defaction :id "layout-flatten-tree" :gloss "Layout flatten tree"
  :category :layout :kind :mutate :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #f))
  :observed ((defobserved :domain :layout :field "tree")))

(defaction :id "layout-set-gaps" :gloss "Layout set gaps"
  :category :layout :kind :mutate :auth :l1
  :params ((defparam :name "inner_h" :kind :str :required #t)
           (defparam :name "inner_v" :kind :str :required #t)
           (defparam :name "outer_l" :kind :str :required #t)
           (defparam :name "outer_r" :kind :str :required #t)
           (defparam :name "outer_t" :kind :str :required #t)
           (defparam :name "outer_b" :kind :int :required #t))
  :observed ((defobserved :domain :layout :field "gaps")))

(defaction :id "layout-set-default-root" :gloss "Layout set default root"
  :category :layout :kind :mutate :auth :l1
  :params ((defparam :name "layout" :kind :str :required #t)
           (defparam :name "orientation" :kind :str :required #t))
  :observed ((defobserved :domain :layout :field "root")))

(defaction :id "layout-get" :gloss "Layout get"
  :category :layout :kind :observe :auth :l1
  :params ((defparam :name "workspace" :kind :selector :required #f))
  :observed ((defobserved :domain :layout :field "get")))

(defaction :id "output-set-enabled" :gloss "Output set enabled"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "enabled" :kind :bool :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :outputs :field "enabled")))

(defaction :id "output-set-mode" :gloss "Output set mode"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "output" :kind :selector :required #f)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :outputs :field "mode")))

(defaction :id "output-set-scale" :gloss "Output set scale"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "scale" :kind :int :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :outputs :field "scale")))

(defaction :id "output-set-transform" :gloss "Output set transform"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "transform" :kind :str :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :outputs :field "transform")))

(defaction :id "output-set-position" :gloss "Output set position"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "position" :kind :str :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :outputs :field "position")))

(defaction :id "output-set-adaptive-sync" :gloss "Output set adaptive sync"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "output" :kind :selector :required #f)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :outputs :field "sync")))

(defaction :id "output-set-power" :gloss "Output set power"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "power" :kind :str :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :outputs :field "power")))

(defaction :id "output-set-gamma" :gloss "Output set gamma"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "correction" :kind :str :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ())

(defaction :id "output-apply-transaction" :gloss "Output apply transaction"
  :category :output :kind :mutate :auth :l2
  :params ((defparam :name "changes" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :outputs :field "transaction")))

(defaction :id "output-list" :gloss "Output list"
  :category :output :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :outputs :field "list")))

(defaction :id "input-set-keyboard-repeat" :gloss "Input set keyboard repeat"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "rate_hz" :kind :str :required #t)
           (defparam :name "delay_ms" :kind :str :required #t))
  :observed ())

(defaction :id "input-set-keyboard-layout" :gloss "Input set keyboard layout"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "layouts" :kind :str :required #t)
           (defparam :name "options" :kind :str :required #t)
           (defparam :name "model" :kind :str :required #t)
           (defparam :name "variant" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "layout")))

(defaction :id "input-switch-keyboard-layout" :gloss "Input switch keyboard layout"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "selector" :kind :selector :required #t))
  :observed ((defobserved :domain :inputs :field "layout")))

(defaction :id "input-set-pointer-accel" :gloss "Input set pointer accel"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "device" :kind :selector :required #t)
           (defparam :name "profile" :kind :str :required #t)
           (defparam :name "speed" :kind :str :required #t))
  :observed ())

(defaction :id "input-set-natural-scroll" :gloss "Input set natural scroll"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "device" :kind :str :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ())

(defaction :id "input-set-tap-to-click" :gloss "Input set tap to click"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "device" :kind :str :required #t)
           (defparam :name "on" :kind :bool :required #t)
           (defparam :name "drag_lock" :kind :str :required #t))
  :observed ())

(defaction :id "input-set-cursor" :gloss "Input set cursor"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "theme" :kind :str :required #t)
           (defparam :name "size" :kind :str :required #t))
  :observed ())

(defaction :id "input-list-devices" :gloss "Input list devices"
  :category :input :kind :observe :auth :l1
  :params ()
  :observed ((defobserved :domain :inputs :field "devices")))

(defaction :id "input-get-state" :gloss "Input get state"
  :category :input :kind :observe :auth :l1
  :params ()
  :observed ((defobserved :domain :inputs :field "state")))

(defaction :id "input-bind" :gloss "Input bind"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "chord" :kind :str :required #t)
           (defparam :name "action" :kind :str :required #t)
           (defparam :name "condition" :kind :str :required #t)
           (defparam :name "rank" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "bindings")))

(defaction :id "input-unbind" :gloss "Input unbind"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "chord" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "bindings")))

(defaction :id "input-set-mode" :gloss "Input set mode"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "mode")))

(defaction :id "input-remap" :gloss "Input remap"
  :category :input :kind :mutate :auth :l2
  :params ((defparam :name "source" :kind :str :required #t)
           (defparam :name "to" :kind :str :required #t)
           (defparam :name "condition" :kind :str :required #t))
  :observed ((defobserved :domain :inputs :field "remap")))

(defaction :id "input-unremap" :gloss "Input unremap"
  :category :input :kind :mutate :auth :l2
  :params ((defparam :name "source" :kind :str :required #t))
  :observed ((defobserved :domain :inputs :field "unremap")))

(defaction :id "input-bind-sequence" :gloss "Input bind sequence"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "chords" :kind :str :required #t)
           (defparam :name "action" :kind :str :required #t)
           (defparam :name "timeout_ms" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "sequence")))

(defaction :id "input-chord-cancel" :gloss "Input chord cancel"
  :category :input :kind :mutate :auth :l1
  :params ()
  :observed ((defobserved :domain :inputs :field "cancel")))

(defaction :id "input-set-leader-timeout" :gloss "Input set leader timeout"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "timeout_ms" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "timeout")))

(defaction :id "input-grab-keyboard" :gloss "Input grab keyboard"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "exclusive" :kind :str :required #t)
           (defparam :name "scope" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "keyboard")))

(defaction :id "input-release-keyboard" :gloss "Input release keyboard"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "scope" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "keyboard")))

(defaction :id "bindings-list" :gloss "Bindings list"
  :category :input :kind :observe :auth :l0
  :params ((defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "list")))

(defaction :id "bindings-check-conflicts" :gloss "Bindings check conflicts"
  :category :input :kind :observe :auth :l0
  :params ((defparam :name "scope" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "conflicts")))

(defaction :id "input-inject-key" :gloss "Input inject key"
  :category :input :kind :mutate :auth :l3
  :params ((defparam :name "key" :kind :str :required #t)
           (defparam :name "state" :kind :str :required #t)
           (defparam :name "modifiers" :kind :str :required #t))
  :observed ())

(defaction :id "input-inject-chord" :gloss "Input inject chord"
  :category :input :kind :mutate :auth :l3
  :params ((defparam :name "chord" :kind :str :required #t))
  :observed ())

(defaction :id "input-inject-text" :gloss "Input inject text"
  :category :input :kind :mutate :auth :l3
  :params ((defparam :name "text" :kind :str :required #t)
           (defparam :name "method" :kind :str :required #t))
  :observed ())

(defaction :id "input-inject-pointer" :gloss "Input inject pointer"
  :category :input :kind :mutate :auth :l3
  :params ((defparam :name "motion" :kind :str :required #t)
           (defparam :name "button" :kind :str :required #t)
           (defparam :name "axis" :kind :str :required #t))
  :observed ())

(defaction :id "gesture-bind" :gloss "Gesture bind"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "gesture" :kind :str :required #t)
           (defparam :name "action" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "bind")))

(defaction :id "gesture-unbind" :gloss "Gesture unbind"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "gesture" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :inputs :field "unbind")))

(defaction :id "gesture-set-suppression" :gloss "Gesture set suppression"
  :category :input :kind :mutate :auth :l1
  :params ((defparam :name "kind" :kind :str :required #t)
           (defparam :name "five_finger_pinch" :kind :str :required #t)
           (defparam :name "five_finger_spread" :kind :str :required #t)
           (defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :inputs :field "suppression")))

(defaction :id "gesture-list" :gloss "Gesture list"
  :category :input :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :inputs :field "list")))

(defaction :id "focus-set-follows-mouse" :gloss "Focus set follows mouse"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :focus :field "mouse")))

(defaction :id "focus-set-mouse-follows-focus" :gloss "Focus set mouse follows focus"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :focus :field "focus")))

(defaction :id "focus-set-auto-center" :gloss "Focus set auto center"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :focus :field "center")))

(defaction :id "focus-set-skip-reshuffle" :gloss "Focus set skip reshuffle"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :focus :field "reshuffle")))

(defaction :id "focus-set-manage-toggle-enabled" :gloss "Focus set manage toggle enabled"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :focus :field "enabled")))

(defaction :id "edge-snap-set-left" :gloss "Edge snap set left"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :focus :field "left")))

(defaction :id "edge-snap-set-right" :gloss "Edge snap set right"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :focus :field "right")))

(defaction :id "edge-snap-set-preview" :gloss "Edge snap set preview"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :focus :field "preview")))

(defaction :id "edge-snap-set-sticky-dwell" :gloss "Edge snap set sticky dwell"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "ms" :kind :str :required #t))
  :observed ((defobserved :domain :focus :field "dwell")))

(defaction :id "focus-policy-get" :gloss "Focus policy get"
  :category :focus :kind :observe :auth :l1
  :params ()
  :observed ((defobserved :domain :focus :field "get")))

(defaction :id "pointer-warp-to" :gloss "Pointer warp to"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "anchor" :kind :str :required #t)
           (defparam :name "monitor_center" :kind :str :required #t)
           (defparam :name "window_center" :kind :str :required #t))
  :observed ())

(defaction :id "rule-add" :gloss "Rule add"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "criteria" :kind :selector :required #t)
           (defparam :name "trigger" :kind :str :required #t)
           (defparam :name "effects" :kind :str :required #t)
           (defparam :name "assertiveness" :kind :str :required #t))
  :observed ((defobserved :domain :focus :field "add")))

(defaction :id "rule-remove" :gloss "Rule remove"
  :category :focus :kind :mutate :auth :l1
  :params ((defparam :name "id" :kind :str :required #t))
  :observed ((defobserved :domain :focus :field "remove")))

(defaction :id "rule-list" :gloss "Rule list"
  :category :focus :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :focus :field "list")))

(defaction :id "hook-set" :gloss "Hook set"
  :category :system :kind :mutate :auth :l1
  :params ((defparam :name "trigger" :kind :str :required #t)
           (defparam :name "argv" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "set")))

(defaction :id "hook-unset" :gloss "Hook unset"
  :category :system :kind :mutate :auth :l1
  :params ((defparam :name "trigger" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "unset")))

(defaction :id "hook-list" :gloss "Hook list"
  :category :system :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "list")))

(defaction :id "decoration-set-border" :gloss "Decoration set border"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "active" :kind :str :required #t)
           (defparam :name "inactive" :kind :str :required #t)
           (defparam :name "width" :kind :int :required #t))
  :observed ((defobserved :domain :windows :field "border")))

(defaction :id "decoration-get" :gloss "Decoration get"
  :category :window :kind :observe :auth :l1
  :params ()
  :observed ((defobserved :domain :windows :field "get")))

(defaction :id "session-lock" :gloss "Session lock"
  :category :session :kind :mutate :auth :l1
  :params ((defparam :name "locker" :kind :str :required #t)
           (defparam :name "grace_ms" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "locked")))

(defaction :id "session-unlock" :gloss "Session unlock"
  :category :session :kind :mutate :auth :l1
  :params ((defparam :name "proof" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "locked")))

(defaction :id "session-logout" :gloss "Session logout"
  :category :session :kind :mutate :auth :l2
  :params ((defparam :name "session" :kind :str :required #t)
           (defparam :name "teardown" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "logout")))

(defaction :id "session-suspend" :gloss "Session suspend"
  :category :session :kind :mutate :auth :l2
  :params ((defparam :name "force" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "suspend")))

(defaction :id "session-reboot" :gloss "Session reboot"
  :category :session :kind :mutate :auth :l3
  :params ((defparam :name "delay" :kind :int :required #t)
           (defparam :name "confirm" :kind :str :required #t))
  :observed ())

(defaction :id "session-poweroff" :gloss "Session poweroff"
  :category :session :kind :mutate :auth :l3
  :params ((defparam :name "delay" :kind :int :required #t)
           (defparam :name "confirm" :kind :str :required #t))
  :observed ())

(defaction :id "session-switch-user" :gloss "Session switch user"
  :category :session :kind :mutate :auth :l2
  :params ((defparam :name "to" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "user")))

(defaction :id "session-switch-vt" :gloss "Session switch vt"
  :category :session :kind :mutate :auth :l2
  :params ((defparam :name "vt" :kind :int :required #t)
           (defparam :name "witness" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "vt")))

(defaction :id "session-set-idle-inhibit" :gloss "Session set idle inhibit"
  :category :session :kind :mutate :auth :l0
  :params ((defparam :name "on" :kind :bool :required #t)
           (defparam :name "reason" :kind :str :required #f)
           (defparam :name "scope" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "inhibit")))

(defaction :id "session-set-idle-policy" :gloss "Session set idle policy"
  :category :session :kind :mutate :auth :l1
  :params ((defparam :name "rungs" :kind :str :required #t)
           (defparam :name "action" :kind :str :required #t))
  :observed ())

(defaction :id "session-observe-idle" :gloss "Session observe idle"
  :category :session :kind :observe :auth :l0
  :params ((defparam :name "seat" :kind :str :required #t)
           (defparam :name "timeout" :kind :int :required #f))
  :observed ((defobserved :domain :session :field "idle")))

(defaction :id "session-list" :gloss "Session list"
  :category :session :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "list")))

(defaction :id "session-get-state" :gloss "Session get state"
  :category :session :kind :observe :auth :l0
  :params ((defparam :name "session" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "state")))

(defaction :id "login-enumerate-principals" :gloss "Login enumerate principals"
  :category :login :kind :observe :auth :l2
  :params ((defparam :name "seat_answer_vec_publicprofile" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "principals")))

(defaction :id "login-resolve-principal" :gloss "Login resolve principal"
  :category :login :kind :observe :auth :l2
  :params ((defparam :name "name_answer_principal" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "principal")))

(defaction :id "login-read-public-profile" :gloss "Login read public profile"
  :category :login :kind :observe :auth :l2
  :params ((defparam :name "principal_answer_publicprofile" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "profile")))

(defaction :id "login-auth-begin" :gloss "Login auth begin"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "service" :kind :str :required #t)
           (defparam :name "principal" :kind :str :required #t)
           (defparam :name "tty" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "begin")))

(defaction :id "login-auth-next" :gloss "Login auth next"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "handle_pamstep" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "next")))

(defaction :id "login-auth-answer" :gloss "Login auth answer"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "handle" :kind :str :required #t)
           (defparam :name "response" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "answer")))

(defaction :id "login-auth-account-check" :gloss "Login auth account check"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "handle_acctverdict" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "check")))

(defaction :id "login-auth-change-token" :gloss "Login auth change token"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "handle" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "token")))

(defaction :id "login-set-credentials" :gloss "Login set credentials"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "handle" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ())

(defaction :id "login-put-env" :gloss "Login put env"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "handle" :kind :str :required #t)
           (defparam :name "key" :kind :str :required #t)
           (defparam :name "value" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "env")))

(defaction :id "login-get-env" :gloss "Login get env"
  :category :login :kind :observe :auth :l2
  :params ((defparam :name "handle" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "env")))

(defaction :id "login-open-session" :gloss "Login open session"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "handle" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "session")))

(defaction :id "login-close-session" :gloss "Login close session"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "handle" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "session")))

(defaction :id "login-auth-end" :gloss "Login auth end"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "handle" :kind :str :required #t)
           (defparam :name "status" :kind :str :required #t))
  :observed ())

(defaction :id "login-mint-capability" :gloss "Login mint capability"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "proof" :kind :str :required #t)
           (defparam :name "seat" :kind :str :required #t)
           (defparam :name "handle_seatcapability_authenticated" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "capability")))

(defaction :id "login-start-session" :gloss "Login start session"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "cap" :kind :str :required #t)
           (defparam :name "session" :kind :selector :required #t))
  :observed ((defobserved :domain :session :field "session")))

(defaction :id "login-autologin" :gloss "Login autologin"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "user" :kind :str :required #t)
           (defparam :name "session" :kind :str :required #t)
           (defparam :name "runfile" :kind :str :required #t)
           (defparam :name "budget" :kind :str :required #t)
           (defparam :name "on_exhausted" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "autologin")))

(defaction :id "login-fork-session" :gloss "Login fork session"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "argv" :kind :str :required #t)
           (defparam :name "env" :kind :str :required #t)
           (defparam :name "uid" :kind :str :required #t)
           (defparam :name "gid" :kind :str :required #t)
           (defparam :name "loginuid" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "session")))

(defaction :id "login-signal-session" :gloss "Login signal session"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "pid" :kind :str :required #t)
           (defparam :name "signal" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "session")))

(defaction :id "login-reap-session" :gloss "Login reap session"
  :category :login :kind :observe :auth :l2
  :params ((defparam :name "pid_exitstatus" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "session")))

(defaction :id "login-handoff" :gloss "Login handoff"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "facts" :kind :str :required #t)
           (defparam :name "value" :kind :str :required #t)
           (defparam :name "epoch" :kind :str :required #t)
           (defparam :name "volatility" :kind :str :required #t)
           (defparam :name "scope" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "handoff")))

(defaction :id "seat-open" :gloss "Seat open"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "seat" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "open")))

(defaction :id "seat-take-control" :gloss "Seat take control"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "seat_uncontrolled_controlled" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "control")))

(defaction :id "seat-open-device" :gloss "Seat open device"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "controlled" :kind :str :required #t)
           (defparam :name "path_devicefd" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "device")))

(defaction :id "seat-close-device" :gloss "Seat close device"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "controlled" :kind :str :required #t)
           (defparam :name "fd" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "device")))

(defaction :id "seat-poll" :gloss "Seat poll"
  :category :login :kind :observe :auth :l2
  :params ((defparam :name "controlled" :kind :str :required #t)
           (defparam :name "deadline_seattransition" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "poll")))

(defaction :id "seat-ack-disable" :gloss "Seat ack disable"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "disabling_disabled" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "disable")))

(defaction :id "seat-reacquire" :gloss "Seat reacquire"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "disabled_controlled" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "reacquire")))

(defaction :id "vt-query-free" :gloss "Vt query free"
  :category :login :kind :observe :auth :l2
  :params ((defparam :name "seat0witness_vtnumber" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "free")))

(defaction :id "vt-set-kd-mode" :gloss "Vt set kd mode"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "witness" :kind :str :required #t)
           (defparam :name "vt" :kind :int :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "mode")))

(defaction :id "vt-set-kb-mode" :gloss "Vt set kb mode"
  :category :login :kind :mutate :auth :l2
  :params ((defparam :name "witness" :kind :str :required #t)
           (defparam :name "vt" :kind :int :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "mode")))

(defaction :id "launch-spawn" :gloss "Launch spawn"
  :category :launch :kind :mutate :auth :l0
  :params ((defparam :name "argv" :kind :str :required #t)
           (defparam :name "env" :kind :str :required #t)
           (defparam :name "cwd" :kind :str :required #t))
  :observed ())

(defaction :id "launch-activate" :gloss "Launch activate"
  :category :launch :kind :mutate :auth :l0
  :params ((defparam :name "app_id" :kind :str :required #t)
           (defparam :name "token" :kind :str :required #t))
  :observed ())

(defaction :id "launch-desktop-entry" :gloss "Launch desktop entry"
  :category :launch :kind :mutate :auth :l0
  :params ((defparam :name "entry" :kind :str :required #t)
           (defparam :name "action" :kind :str :required #t)
           (defparam :name "args" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "entry")))

(defaction :id "launch-open-url" :gloss "Launch open url"
  :category :launch :kind :mutate :auth :l0
  :params ((defparam :name "url" :kind :str :required #t))
  :observed ())

(defaction :id "launch-open-path" :gloss "Launch open path"
  :category :launch :kind :mutate :auth :l0
  :params ((defparam :name "with" :kind :str :required #t)
           (defparam :name "path" :kind :str :required #f))
  :observed ())

(defaction :id "launch-focus-or-spawn" :gloss "Launch focus or spawn"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "criteria" :kind :selector :required #t)
           (defparam :name "argv" :kind :str :required #t)
           (defparam :name "timeout_ms" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "spawn")))

(defaction :id "launcher-toggle" :gloss "Launcher toggle"
  :category :launch :kind :mutate :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "toggle")))

(defaction :id "launcher-show" :gloss "Launcher show"
  :category :launch :kind :mutate :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "show")))

(defaction :id "launcher-hide" :gloss "Launcher hide"
  :category :launch :kind :mutate :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "hide")))

(defaction :id "launcher-set-query" :gloss "Launcher set query"
  :category :launch :kind :mutate :auth :l0
  :params ((defparam :name "query" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "query")))

(defaction :id "launcher-get-state" :gloss "Launcher get state"
  :category :launch :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "state")))

(defaction :id "launcher-run-command" :gloss "Launcher run command"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "argv" :kind :str :required #t))
  :observed ())

(defaction :id "clipboard-get" :gloss "Clipboard get"
  :category :clipboard :kind :observe :auth :l2
  :params ((defparam :name "mime_answer_bytes" :kind :str :required #t)
           (defparam :name "selection" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "get")))

(defaction :id "clipboard-put" :gloss "Clipboard put"
  :category :clipboard :kind :mutate :auth :l2
  :params ((defparam :name "mime" :kind :str :required #t)
           (defparam :name "data" :kind :str :required #t)
           (defparam :name "sensitive" :kind :bool :required #t)
           (defparam :name "selection" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "put")))

(defaction :id "clipboard-clear" :gloss "Clipboard clear"
  :category :clipboard :kind :mutate :auth :l2
  :params ((defparam :name "selection" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "clear")))

(defaction :id "clipboard-history-list" :gloss "Clipboard history list"
  :category :clipboard :kind :observe :auth :l1
  :params ((defparam :name "limit" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "list")))

(defaction :id "clipboard-history-clear" :gloss "Clipboard history clear"
  :category :clipboard :kind :mutate :auth :l1
  :params ()
  :observed ((defobserved :domain :session :field "clear")))

(defaction :id "screenshot-capture" :gloss "Screenshot capture"
  :category :capture :kind :mutate :auth :l2
  :params ((defparam :name "source" :kind :str :required #t)
           (defparam :name "sink" :kind :str :required #t)
           (defparam :name "include_cursor" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "capture")))

(defaction :id "screenshot-pick-region" :gloss "Screenshot pick region"
  :category :capture :kind :observe :auth :l1
  :params ((defparam :name "prompt" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "region")))

(defaction :id "color-pick" :gloss "Color pick"
  :category :capture :kind :observe :auth :l1
  :params ()
  :observed ((defobserved :domain :session :field "pick")))

(defaction :id "screencast-start" :gloss "Screencast start"
  :category :capture :kind :mutate :auth :l2
  :params ((defparam :name "source" :kind :str :required #t)
           (defparam :name "sink" :kind :str :required #t)
           (defparam :name "fps" :kind :str :required #t)
           (defparam :name "include_cursor_castid" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "start")))

(defaction :id "screencast-stop" :gloss "Screencast stop"
  :category :capture :kind :mutate :auth :l2
  :params ((defparam :name "cast" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "stop")))

(defaction :id "screencast-list" :gloss "Screencast list"
  :category :capture :kind :observe :auth :l1
  :params ()
  :observed ((defobserved :domain :session :field "list")))

(defaction :id "notify-send" :gloss "Notify send"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "summary" :kind :str :required #t)
           (defparam :name "app_id" :kind :str :required #t)
           (defparam :name "timeout_ms" :kind :str :required #t)
           (defparam :name "actions" :kind :str :required #t)
           (defparam :name "icon_notificationid" :kind :str :required #t)
           (defparam :name "body" :kind :str :required #f)
           (defparam :name "urgency" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "send")))

(defaction :id "notify-invoke-action" :gloss "Notify invoke action"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "id" :kind :str :required #t)
           (defparam :name "action_id" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "action")))

(defaction :id "notify-dismiss" :gloss "Notify dismiss"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "id" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "dismiss")))

(defaction :id "notify-dismiss-all" :gloss "Notify dismiss all"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "filter" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "all")))

(defaction :id "notify-mark-read" :gloss "Notify mark read"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "target" :kind :selector :required #t))
  :observed ((defobserved :domain :session :field "read")))

(defaction :id "notify-set-dnd" :gloss "Notify set dnd"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "on" :kind :bool :required #t)
           (defparam :name "until" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "dnd")))

(defaction :id "notify-list" :gloss "Notify list"
  :category :notify :kind :observe :auth :l0
  :params ((defparam :name "include_history" :kind :str :required #t)
           (defparam :name "filter" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "list")))

(defaction :id "notify-history-clear" :gloss "Notify history clear"
  :category :notify :kind :mutate :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "clear")))

(defaction :id "notify-history-delete-one" :gloss "Notify history delete one"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "entry_id" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "one")))

(defaction :id "notify-toggle-expand" :gloss "Notify toggle expand"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "id" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "expand")))

(defaction :id "notify-toggle-view" :gloss "Notify toggle view"
  :category :notify :kind :mutate :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "view")))

(defaction :id "notify-search" :gloss "Notify search"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "query" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "search")))

(defaction :id "notify-filter-set" :gloss "Notify filter set"
  :category :notify :kind :mutate :auth :l0
  :params ((defparam :name "by" :kind :str :required #t)
           (defparam :name "value" :kind :str :required #t)
           (defparam :name "urgency" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "set")))

(defaction :id "layer-spawn" :gloss "Layer spawn"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "client" :kind :str :required #t)
           (defparam :name "layer" :kind :str :required #t)
           (defparam :name "anchor" :kind :str :required #t)
           (defparam :name "exclusive_zone" :kind :str :required #t)
           (defparam :name "keyboard" :kind :str :required #t)
           (defparam :name "output" :kind :selector :required #f))
  :observed ((defobserved :domain :windows :field "spawn")))

(defaction :id "layer-kill" :gloss "Layer kill"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "client" :kind :str :required #t)
           (defparam :name "teardown" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "kill")))

(defaction :id "layer-list" :gloss "Layer list"
  :category :window :kind :observe :auth :l1
  :params ()
  :observed ((defobserved :domain :windows :field "list")))

(defaction :id "pane-session-new" :gloss "Pane session new"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "name_panesessionid" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "fresh")))

(defaction :id "pane-session-kill" :gloss "Pane session kill"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "session" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "kill")))

(defaction :id "pane-session-list" :gloss "Pane session list"
  :category :window :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "list")))

(defaction :id "pane-session-detail" :gloss "Pane session detail"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "session" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "detail")))

(defaction :id "pane-list" :gloss "Pane list"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "session" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "list")))

(defaction :id "pane-snapshot" :gloss "Pane snapshot"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "pane" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "snapshot")))

(defaction :id "pane-send-keys" :gloss "Pane send keys"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "pane" :kind :str :required #t)
           (defparam :name "keys" :kind :str :required #t))
  :observed ())

(defaction :id "pane-set-input-policy" :gloss "Pane set input policy"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "pane" :kind :str :required #t)
           (defparam :name "policy" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "policy")))

(defaction :id "pane-subscriber-count" :gloss "Pane subscriber count"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "pane" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "count")))

(defaction :id "pane-record-start" :gloss "Pane record start"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "pane" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "start")))

(defaction :id "pane-record-stop" :gloss "Pane record stop"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "pane" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "stop")))

(defaction :id "pane-record-export" :gloss "Pane record export"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "pane" :kind :str :required #t)
           (defparam :name "path" :kind :str :required #f))
  :observed ((defobserved :domain :windows :field "export")))

(defaction :id "pane-record-status" :gloss "Pane record status"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "pane" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "status")))

(defaction :id "pane-blocks-list" :gloss "Pane blocks list"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "pane" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "list")))

(defaction :id "pane-block-at" :gloss "Pane block at"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "pane" :kind :str :required #t)
           (defparam :name "index" :kind :int :required #f))
  :observed ((defobserved :domain :windows :field "at")))

(defaction :id "pane-blocks-status" :gloss "Pane blocks status"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "pane" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "status")))

(defaction :id "pane-stats" :gloss "Pane stats"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "pane" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "stats")))

(defaction :id "pane-top" :gloss "Pane top"
  :category :window :kind :observe :auth :l0
  :params ((defparam :name "limit" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "top")))

(defaction :id "pane-daemon-status" :gloss "Pane daemon status"
  :category :window :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "status")))

(defaction :id "pane-system-resources" :gloss "Pane system resources"
  :category :window :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "resources")))

(defaction :id "pane-socket-info" :gloss "Pane socket info"
  :category :window :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "info")))

(defaction :id "pane-config-get" :gloss "Pane config get"
  :category :window :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "get")))

(defaction :id "pane-config-set" :gloss "Pane config set"
  :category :window :kind :mutate :auth :l1
  :params ((defparam :name "yaml" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "set")))

(defaction :id "pane-reload-config" :gloss "Pane reload config"
  :category :window :kind :mutate :auth :l1
  :params ()
  :observed ((defobserved :domain :windows :field "config")))

(defaction :id "browser-open" :gloss "Browser open"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "url_browserid" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "open")))

(defaction :id "browser-close" :gloss "Browser close"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "id" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "close")))

(defaction :id "browser-focus" :gloss "Browser focus"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "id" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "focus")))

(defaction :id "browser-list" :gloss "Browser list"
  :category :launch :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :windows :field "list")))

(defaction :id "browser-move" :gloss "Browser move"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "id" :kind :str :required #t)
           (defparam :name "position" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "move")))

(defaction :id "browser-resize" :gloss "Browser resize"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "id" :kind :str :required #t)
           (defparam :name "size" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "resize")))

(defaction :id "browser-navigate" :gloss "Browser navigate"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "id" :kind :str :required #t)
           (defparam :name "url" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "navigate")))

(defaction :id "browser-snap" :gloss "Browser snap"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "id" :kind :str :required #t)
           (defparam :name "zone" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "snap")))

(defaction :id "browser-snapshot" :gloss "Browser snapshot"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "id" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "snapshot")))

(defaction :id "browser-snapshot-get" :gloss "Browser snapshot get"
  :category :launch :kind :observe :auth :l0
  :params ((defparam :name "id" :kind :str :required #t))
  :observed ((defobserved :domain :windows :field "get")))

(defaction :id "browser-set-dom" :gloss "Browser set dom"
  :category :launch :kind :mutate :auth :l1
  :params ((defparam :name "id" :kind :str :required #t)
           (defparam :name "dom" :kind :str :required #t))
  :observed ())

(defaction :id "theme-select" :gloss "Theme select"
  :category :theme :kind :mutate :auth :l0
  :params ((defparam :name "theme" :kind :str :required #t))
  :observed ((defobserved :domain :theme :field "name")))

(defaction :id "theme-set-appearance" :gloss "Theme set appearance"
  :category :theme :kind :mutate :auth :l0
  :params ((defparam :name "appearance" :kind :str :required #t)
           (defparam :name "dark" :kind :str :required #t)
           (defparam :name "auto" :kind :str :required #t))
  :observed ((defobserved :domain :theme :field "appearance")))

(defaction :id "theme-set-font-scale" :gloss "Theme set font scale"
  :category :theme :kind :mutate :auth :l0
  :params ((defparam :name "scale" :kind :int :required #t))
  :observed ((defobserved :domain :theme :field "scale")))

(defaction :id "theme-reload" :gloss "Theme reload"
  :category :theme :kind :mutate :auth :l0
  :params ()
  :observed ((defobserved :domain :theme :field "revision")))

(defaction :id "theme-get" :gloss "Theme get"
  :category :theme :kind :observe :auth :l0
  :params ((defparam :name "theme_per_leaf_provenance" :kind :str :required #t))
  :observed ((defobserved :domain :theme :field "get")))

(defaction :id "screen-transition-run" :gloss "Screen transition run"
  :category :theme :kind :mutate :auth :l1
  :params ((defparam :name "delay_ms" :kind :str :required #t))
  :observed ())

(defaction :id "audio-set-volume" :gloss "Audio set volume"
  :category :system :kind :mutate :auth :l0
  :params ((defparam :name "target" :kind :selector :required #t)
           (defparam :name "sink" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "volume")))

(defaction :id "audio-set-mute" :gloss "Audio set mute"
  :category :system :kind :mutate :auth :l0
  :params ((defparam :name "on" :kind :bool :required #t)
           (defparam :name "sink" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "mute")))

(defaction :id "audio-set-default-sink" :gloss "Audio set default sink"
  :category :system :kind :mutate :auth :l0
  :params ((defparam :name "sink" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "sink")))

(defaction :id "audio-list-sinks" :gloss "Audio list sinks"
  :category :system :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "sinks")))

(defaction :id "screensaver-start" :gloss "Screensaver start"
  :category :system :kind :mutate :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "start")))

(defaction :id "trash-empty" :gloss "Trash empty"
  :category :system :kind :mutate :auth :l3
  :params ((defparam :name "confirm" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "empty")))

(defaction :id "volume-eject-all" :gloss "Volume eject all"
  :category :system :kind :mutate :auth :l1
  :params ()
  :observed ((defobserved :domain :session :field "all")))

(defaction :id "overview-set" :gloss "Overview set"
  :category :system :kind :mutate :auth :l1
  :params ((defparam :name "on" :kind :bool :required #t))
  :observed ((defobserved :domain :session :field "set")))

(defaction :id "config-reload" :gloss "Config reload"
  :category :system :kind :mutate :auth :l1
  :params ((defparam :name "path" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "reload")))

(defaction :id "config-get" :gloss "Config get"
  :category :system :kind :observe :auth :l0
  :params ((defparam :name "key" :kind :str :required #t)
           (defparam :name "include_provenance" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "get")))

(defaction :id "config-set" :gloss "Config set"
  :category :system :kind :mutate :auth :l1
  :params ((defparam :name "key" :kind :str :required #t)
           (defparam :name "value" :kind :str :required #t)
           (defparam :name "apply" :kind :str :required #t)
           (defparam :name "live" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "set")))

(defaction :id "state-query" :gloss "State query"
  :category :system :kind :observe :auth :l1
  :params ((defparam :name "domain" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "query")))

(defaction :id "events-subscribe" :gloss "Events subscribe"
  :category :system :kind :observe :auth :l1
  :params ((defparam :name "since" :kind :str :required #t)
           (defparam :name "filter" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "subscribe")))

(defaction :id "compositor-quit" :gloss "Compositor quit"
  :category :system :kind :mutate :auth :l3
  :params ((defparam :name "confirm" :kind :str :required #t)
           (defparam :name "teardown" :kind :str :required #t))
  :observed ())

(defaction :id "desktop-plan" :gloss "Desktop plan"
  :category :system :kind :observe :auth :l1
  :params ((defparam :name "spec" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "plan")))

(defaction :id "desktop-apply" :gloss "Desktop apply"
  :category :system :kind :mutate :auth :l1
  :params ((defparam :name "spec" :kind :str :required #t)
           (defparam :name "policy" :kind :str :required #t)
           (defparam :name "mode" :kind :str :required #f))
  :observed ((defobserved :domain :session :field "apply")))

(defaction :id "desktop-tick" :gloss "Desktop tick"
  :category :system :kind :mutate :auth :l1
  :params ((defparam :name "tickoutcome" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "tick")))

(defaction :id "desktop-attest" :gloss "Desktop attest"
  :category :system :kind :observe :auth :l0
  :params ((defparam :name "receipt_desktoppayload" :kind :str :required #t))
  :observed ((defobserved :domain :session :field "attest")))

(defaction :id "desktop-health" :gloss "Desktop health"
  :category :system :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "health")))

(defaction :id "desktop-version" :gloss "Desktop version"
  :category :system :kind :observe :auth :l0
  :params ()
  :observed ((defobserved :domain :session :field "version")))
