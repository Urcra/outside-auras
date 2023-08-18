-- The global aura_env is always available and should be used, then we can have some nice debug in the future

-- Handle and parse a log_line and update your own state, return a bool depending on if we should redraw the overlay or not
function handle_log_line(log_line)
    return true
end

-- The required height for the aura depdning on current state, we will resize the window to this size before each redraw should be f32
function required_height()
    return 2 * item_height()
end

-- The required width for the aura depdning on current state, we will resize the window to this size before each redraw should be f32
function required_width()
    return item_width()
end

-- The expected height of each item that you would return, we use this when drawing your items should be f32
function item_height()
    return 56.0
end

-- The expected width of each item that you would return, we use this when drawing your items should be f32
function item_width()
    return 256.0
end

-- Return the items to draw in the aura, should be a list of lists, where the first two elements are the String on the icon, and the content String
function items()
    list_items = {{"1", "Item1FromLua"}, {"2", "Item2FromLua"}}
    return list_items
end

