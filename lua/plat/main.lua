---@diagnostic disable: duplicate-set-field
local Base = require 'lib.char'

local GRAVITY = 50
local JUMP_VELOCITY = -700
local GROUND_LEVEL = 576 - 25

local game = Base.Game(1024, 576, GRAVITY, JUMP_VELOCITY, GROUND_LEVEL)
local character = Base.PlatformerBase(game, Base.Collision(0, 0, 60, 60), 50)


local game_objects = {
  {"rect", Base.Collision(0, GROUND_LEVEL, game.width, game.height - GROUND_LEVEL), "stand"},
  {"tri", Base.Collision(game.width * 0.5, GROUND_LEVEL - 40, 40, 40), "kill"},
  {"tri", Base.Collision(game.width * 0.5 + 40, GROUND_LEVEL - 40, 40, 40), "kill"},
  {"rect", Base.Collision(game.width * 0.5 + 80, GROUND_LEVEL - 40, 40, 40), "stand"}
}

function love.load()
  love.window.setMode(game.width, game.height)
end

function love.update(dt)
  if character:is_touching_wall("left") and character.direction == -1 then
    character.direction = 0
  elseif character:is_touching_wall("right") and character.direction == 1 then
    character.direction = 0
  end

  local touching = false
  for _, object in ipairs(game_objects) do
    local collision = object[2]
    local on_touch = object[3]

    if on_touch == "stand" then
      character:move(dt, touching, collision.y)
    else
      character:move(dt)
    end
  end
end

function love.draw()
  character.collision:draw_rect()
  for _, object in ipairs(game_objects) do
    local shape = object[1]
    local collision = object[2]

    if shape == "tri" then
      collision:draw_tri()
    elseif shape == "rect" then
      collision:draw_rect()
    end
  end
end

function love.keypressed(key)
  if key == "left" then
    character.direction = -1
  elseif key == "right" then
    character.direction = 1
  -- elseif key == "space" and character:is_standing() then
  elseif key == "space" then
    -- character:do_jump()
    character.collision.y = 0
  end
end

function love.keyreleased(key)
  if key == "left" or key == "right" then
    character.direction = 0
  end
end
