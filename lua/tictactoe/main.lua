local ButtonManager = require 'lib.simplebutton'
local array_tools = require 'lib.array_tools'

local love = love;

local screen = {
  w = 500,
  h = 500
}

local board = {{0, 0, 0}, {0, 0, 0}, {0, 0, 0}}
-- local board = array_tools.genlist(0, {3, 3})
local button_board = {{}, {}, {}}

local current_player = 1

local function checkWin(b)
  local conditions = {
    -- rows
    b[1][1] + b[1][2] + b[1][3],
    b[2][1] + b[2][2] + b[2][3],
    b[3][1] + b[3][2] + b[3][3],

    -- cols
    b[1][1] + b[2][1] + b[3][1],
    b[1][2] + b[2][2] + b[3][2],
    b[1][3] + b[2][3] + b[3][3],

    -- diags
    b[1][1] + b[2][2] + b[3][3],
    b[1][3] + b[2][2] + b[3][1]
  }

  if array_tools.has_value(conditions, 3) then
    return "x"
  elseif array_tools.has_value(conditions, -3) then
    return 'o'
  else
    return nil
  end
end

local function buttonClick(row, col) return function()
  if board[row+1][col+1] == 0 then
    board[row+1][col+1] = current_player

    -- make the button say x or o
    if current_player == 1 then
      button_board[row+1][col+1]:setImage('x.png')
    else
      button_board[row+1][col+1]:setImage('o.png')
    end

    current_player = current_player * -1

    if type(checkWin(board)) == "string" then
      print(checkWin(board)," wins")
      love.event.quit(0)

    elseif not array_tools.is_all(board, 0) then
      print("tie")
      love.event.quit(0)
    end
  end
end end

function love.load()
  love.window.setMode(screen.w, screen.h)

  local spacer = 20;
  ButtonManager.default.width = (screen.h - (spacer*6)) / 3
  ButtonManager.default.height = (screen.h - (spacer*6)) / 3

  for row = 0, 2 do for col = 0, 2 do
    local x = spacer + col * (ButtonManager.default.width + 2*spacer)
    local y = spacer + row * (ButtonManager.default.height + 2*spacer)

    button_board[row+1][col+1] = ButtonManager.new("", x, y)

    button_board[row+1][col+1].onClick = buttonClick(row, col)
  end end
end

local bg = love.graphics.newImage('board.png')
local screen_quad = love.graphics.newQuad(0, 0, screen.h, screen.w, screen.h, screen.h)

function love.draw()
  love.graphics.setBackgroundColor(1, 1, 1, 1)
  love.graphics.draw(bg, screen_quad)

  ButtonManager.draw()

end

function love.mousepressed(x, y, msbutton)
   ButtonManager.mousepressed(x, y, msbutton)
end

function love.mousereleased(x, y, msbutton)
   ButtonManager.mousereleased(x, y, msbutton)
end
