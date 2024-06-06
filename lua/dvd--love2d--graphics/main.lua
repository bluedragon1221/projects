---@diagnostic disable: undefined-global

local Object = require 'lib.classic'
local switch = require 'lib.switch'

local screen = {
  width = 1024,
  height = 576
}

local Character = Object:extend()
function Character:new()
  self.width = 150
  self.height = 75

  self.x = 0
  self.y = screen.height - self.height

  self._BASE_VX = 400
  self._BASE_VY = 100

  self.vx = self._BASE_VX
  self.vy = -self._BASE_VY

  self.image = nil
  self.quad = nil

  self.is_moving = false
end

function Character:move(dt)
  self.x = self.x + (self.vx * dt)
  self.y = self.y + (self.vy * dt)
end

function Character:draw()
	love.graphics.draw(self.image, self.quad, self.x, self.y)
end

local Side = {
  Left = {},
  Right = {},
  Top = {},
  Bottom = {}
}

function Character:is_touching_wall()
  if self.x <= 0 then
    return true, Side.Left
  elseif (self.x + self.width) >= screen.width then
    return true, Side.Right
  elseif self.y <= 0 then
    return true, Side.Top
  elseif (self.y + self.height) >= screen.height then
    return true, Side.Bottom
  else
    return false
  end
end

function Character:turn_on_wall(wall)
  switch(wall)
    .case(Side.Left, function() self.vx = self._BASE_VX end)
    .case(Side.Right, function() self.vx = -self._BASE_VX end)
    .case(Side.Top, function() self.vy = self._BASE_VY end)
    .case(Side.Bottom, function() self.vy = -self._BASE_VY end)
    .process()
end

function Character:load_image(file)
  local img = love.graphics.newImage(file)
  self.image = img
  self.quad = love.graphics.newQuad(0, 0, img:getPixelWidth(), img:getPixelHeight(), self.width, self.height)
end

local character = Character()

function love.load()
  character:load_image('dvd.png')
  love.window.setMode(screen.width, screen.height)
end

function love.update(dt)
  if character.is_moving then character:move(dt) end

  local touching, wall = character:is_touching_wall()
  if touching then
    character:turn_on_wall(wall)
  end
end

function love.draw()
  character:draw()
end

function love.mousepressed()
  character.is_moving = true
  character.vx = -character.vx
end
