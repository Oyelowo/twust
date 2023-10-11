#[cfg(feature = "daisyui")]
pub fn get_it() -> Vec<&'static str> {
    DAISY_UI_CLASSES.to_vec()
}

#[cfg(not(feature = "daisyui"))]
pub fn get_it() -> Vec<&'static str> {
    Vec::new()
}

#[cfg(feature = "daisyui")]
pub const DAISY_UI_CLASSES: [&str; 420] = [
    // Collapse
    "collapse",
    "collapse-title",
    "collapse-content",
    "collapse-arrow",
    "collapse-plus",
    "collapse-open",
    "collapse-close",
    // Alert
    "alert",
    "alert-info",
    "alert-success",
    "alert-warning",
    "alert-error",
    //artboard
    "artboard",
    "artboard-demo",
    "artboard-horizontal",
    // phones
    "phone-1",
    "phone-2",
    "phone-3",
    "phone-4",
    "phone-5",
    "phone-6",
    // Avatar
    "avatar",
    "avatar-group",
    "online",
    "offline",
    "placeholder",
    // Badge
    "badge",
    "badge-neutral",
    "badge-primary",
    "badge-secondary",
    "badge-accent",
    "badge-ghost",
    "badge-info",
    "badge-success",
    "badge-warning",
    "badge-error",
    "badge-outline",
    "badge-lg",
    "badge-md",
    "badge-sm",
    "badge-xs",
    // Bottom Navigation
    "btm-nav",
    "active",
    "disabled",
    "btm-nav-xs",
    "btm-nav-sm",
    "btm-nav-md",
    "btm-nav-lg",
    "breadcrumbs",
    // Button
    "btn",
    "btn-neutral",
    "btn-primary",
    "btn-secondary",
    "btn-accent",
    "btn-info",
    "btn-success",
    "btn-warning",
    "btn-error",
    "btn-ghost",
    "btn-link",
    "btn-outline",
    "btn-active",
    "btn-disabled",
    "glass",
    "no-animation",
    "btn-lg",
    "btn-md",
    "btn-sm",
    "btn-xs",
    "btn-wide",
    "btn-block",
    "btn-circle",
    "btn-square",
    "btn-group",
    "btn-group-horizontal",
    "btn-group-vertical",
    // Card
    "card",
    "card-title",
    "card-body",
    "card-actions",
    "card-bordered",
    "image-full",
    "card-normal",
    "card-compact",
    "card-side",
    // Carousel
    "carousel",
    "carousel-item",
    "carousel-center",
    "carousel-end",
    "carousel-vertical",
    // Chat
    "chat",
    "chat-start",
    "chat-end",
    "chat-image",
    "chat-header",
    "chat-footer",
    "chat-bubble",
    "chat-bubble-primary",
    "chat-bubble-secondary",
    "chat-bubble-accent",
    "chat-bubble-info",
    "chat-bubble-success",
    "chat-bubble-warning",
    "chat-bubble-error",
    // Form Control (Checkbox)
    "form-control",
    "checkbox",
    "checkbox-primary",
    "checkbox-secondary",
    "checkbox-accent",
    "checkbox-success",
    "checkbox-warning",
    "checkbox-info",
    "checkbox-error",
    "checkbox-lg",
    "checkbox-md",
    "checkbox-sm",
    "checkbox-xs",
    // Collapse (repeated from above, not adding again)

    // Countdown
    "countdown",
    // Divider
    "divider",
    "divider-vertical",
    "divider-horizontal",
    // Drawer
    "drawer",
    "drawer-toggle",
    "drawer-content",
    "drawer-side",
    "drawer-overlay",
    "drawer-end",
    "drawer-open",
    // Dropdown
    "dropdown",
    "dropdown-content",
    "dropdown-end",
    "dropdown-top",
    "dropdown-bottom",
    "dropdown-left",
    "dropdown-right",
    "dropdown-hover",
    "dropdown-open",
    // Form Control (File Input)
    "label",
    "file-input",
    "file-input-bordered",
    "file-input-ghost",
    "file-input-primary",
    "file-input-secondary",
    "file-input-accent",
    "file-input-info",
    "file-input-success",
    "file-input-warning",
    "file-input-error",
    "file-input-lg",
    "file-input-md",
    "file-input-sm",
    "file-input-xs",
    // Footer
    "footer",
    "footer-title",
    "footer-center",
    // Hero
    "hero",
    "hero-content",
    "hero-overlay",
    // Indicator
    "indicator",
    "indicator-item",
    "indicator-start",
    "indicator-center",
    "indicator-end",
    "indicator-top",
    "indicator-middle",
    "indicator-bottom",
    // Form Control (Input)
    "input",
    "input-bordered",
    "input-ghost",
    "input-primary",
    "input-secondary",
    "input-accent",
    "input-info",
    "input-success",
    "input-warning",
    "input-error",
    "input-lg",
    "input-md",
    "input-sm",
    "input-xs",
    // Form Control (Input Group)
    "input-group",
    "input-group-lg",
    "input-group-md",
    "input-group-sm",
    "input-group-xs",
    "input-group-vertical",
    // Join
    "join",
    "join-item",
    "join-vertical",
    "join-horizontal",
    // Key
    "kbd",
    "kbd-lg",
    "kbd-md",
    "kbd-sm",
    "kbd-xs",
    // Link
    "link",
    "link-neutral",
    "link-primary",
    "link-secondary",
    "link-accent",
    "link-success",
    "link-info",
    "link-warning",
    "link-error",
    "link-hover",
    // Loader
    "loading",
    "loading-spinner",
    "loading-dots",
    "loading-ring",
    "loading-ball",
    "loading-bars",
    "loading-infinity",
    // Mask
    "mask",
    "mask-squircle",
    "mask-heart",
    "mask-hexagon",
    "mask-hexagon-2",
    "mask-decagon",
    "mask-pentagon",
    "mask-diamond",
    "mask-square",
    "mask-circle",
    "mask-parallelogram",
    "mask-parallelogram-2",
    "mask-parallelogram-3",
    "mask-parallelogram-4",
    "mask-star",
    "mask-star-2",
    "mask-triangle",
    "mask-triangle-2",
    "mask-triangle-3",
    "mask-triangle-4",
    "mask-half-1",
    "mask-half-2",
    // Menu
    "menu",
    "menu-title",
    "disabled",
    "active",
    "focus",
    "menu-dropdown-toggle",
    "menu-dropdown",
    "menu-dropdown-show",
    "menu-xs",
    "menu-sm",
    "menu-md",
    "menu-lg",
    "menu-vertical",
    "menu-horizontal",
    // Mockup
    "mockup-browser",
    "mockup-browser-toolbar",
    "mockup-code",
    "mockup-phone",
    "mockup-window",
    // Modal
    "modal",
    "modal-box",
    "modal-action",
    "modal-backdrop",
    "modal-toggle",
    "modal-open",
    "modal-top",
    "modal-bottom",
    "modal-middle",
    // Navbar
    "navbar",
    "navbar-start",
    "navbar-center",
    "navbar-end",
    "join",
    // Progress
    "progress",
    "progress-primary",
    "progress-secondary",
    "progress-accent",
    "progress-info",
    "progress-success",
    "progress-warning",
    "progress-error",
    "radial-progress",
    // Form Control (Radio)
    "form-control",
    // Radio
    "radio",
    "radio-primary",
    "radio-secondary",
    "radio-accent",
    "radio-success",
    "radio-warning",
    "radio-info",
    "radio-error",
    "radio-lg",
    "radio-md",
    "radio-sm",
    "radio-xs",
    // Range
    "range",
    "range-primary",
    "range-secondary",
    "range-accent",
    "range-success",
    "range-warning",
    "range-info",
    "range-error",
    "range-lg",
    "range-md",
    "range-sm",
    "range-xs",
    // Rating
    "rating",
    "rating-half",
    "rating-hidden",
    "rating-lg",
    "rating-md",
    "rating-sm",
    "rating-xs",
    "form-control",
    "label",
    // Select
    "select",
    "select-bordered",
    "select-ghost",
    "select-primary",
    "select-secondary",
    "select-accent",
    "select-info",
    "select-success",
    "select-warning",
    "select-error",
    "select-lg",
    "select-md",
    "select-sm",
    "select-xs",
    // Stack
    "stack",
    // Stat
    "stats",
    "stat",
    "stat-title",
    "stat-value",
    "stat-desc",
    "stat-figure",
    "stat-actions",
    "stats-horizontal",
    "stats-vertical",
    // Steps
    "steps",
    "step",
    "step-primary",
    "step-secondary",
    "step-accent",
    "step-info",
    "step-success",
    "step-warning",
    "step-error",
    "steps-vertical",
    "steps-horizontal",
    // Swap
    "swap",
    "swap-on",
    "swap-off",
    "swap-indeterminate",
    "swap-active",
    "swap-rotate",
    "swap-flip",
    // Tabs
    "tabs",
    "tabs-boxed",
    "tab",
    "tab-active",
    "tab-disabled",
    "tab-bordered",
    "tab-lifted",
    "tab-xs",
    "tab-sm",
    "tab-md",
    "tab-lg",
    // Table
    "table",
    "table-zebra",
    "table-pin-rows",
    "table-pin-cols",
    "table-xs",
    "table-sm",
    "table-md",
    "table-lg",
    "form-control",
    "label",
    // Textarea
    "textarea",
    "textarea-bordered",
    "textarea-ghost",
    "textarea-primary",
    "textarea-secondary",
    "textarea-accent",
    "textarea-info",
    "textarea-success",
    "textarea-warning",
    "textarea-error",
    "textarea-lg",
    "textarea-md",
    "textarea-sm",
    "textarea-xs",
    // Toast
    "toast",
    "toast-start",
    "toast-center",
    "toast-end",
    "toast-top",
    "toast-middle",
    "toast-bottom",
    "form-control",
    // Toggle
    "toggle",
    "toggle-primary",
    "toggle-secondary",
    "toggle-accent",
    "toggle-success",
    "toggle-warning",
    "toggle-info",
    "toggle-error",
    "toggle-lg",
    "toggle-md",
    "toggle-sm",
    "toggle-xs",
    // Tooltip
    "tooltip",
    "tooltip-open",
    "tooltip-top",
    "tooltip-bottom",
    "tooltip-left",
    "tooltip-right",
    "tooltip-primary",
    "tooltip-secondary",
    "tooltip-accent",
    "tooltip-info",
    "tooltip-success",
    "tooltip-warning",
    "tooltip-error",
];