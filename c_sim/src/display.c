/*
 * Copyright (C) 2024 ZhaoCake
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#include "display.h"

// 寄存器访问宏
#define DISPLAY_REG(offset) (*((volatile uint32_t *)(DISPLAY_BASE + (offset))))

void display_init(void) {
    DISPLAY_REG(DISPLAY_CTRL) = DISPLAY_CTRL_ENABLE;
}

void display_enable(void) {
    DISPLAY_REG(DISPLAY_CTRL) |= DISPLAY_CTRL_ENABLE;
}

void display_disable(void) {
    DISPLAY_REG(DISPLAY_CTRL) &= ~DISPLAY_CTRL_ENABLE;
}

void display_set_pixel(uint32_t x, uint32_t y, uint32_t color) {
    if (x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT) {
        return;
    }
    DISPLAY_REG(DISPLAY_X) = x;
    DISPLAY_REG(DISPLAY_Y) = y;
    DISPLAY_REG(DISPLAY_COLOR) = color;
}

void display_update(void) {
    DISPLAY_REG(DISPLAY_UPDATE) = 1;
}

void display_clear(uint32_t color) {
    for (uint32_t y = 0; y < DISPLAY_HEIGHT; y++) {
        for (uint32_t x = 0; x < DISPLAY_WIDTH; x++) {
            display_set_pixel(x, y, color);
        }
    }
    display_update();
}

// Bresenham's line algorithm
void display_draw_line(uint32_t x1, uint32_t y1, uint32_t x2, uint32_t y2, uint32_t color) {
    int dx = (int)x2 - (int)x1;
    int dy = (int)y2 - (int)y1;
    int dx1 = dx < 0 ? -dx : dx;
    int dy1 = dy < 0 ? -dy : dy;
    int px = dx < 0 ? -1 : 1;
    int py = dy < 0 ? -1 : 1;
    int x = x1, y = y1;
    
    display_set_pixel(x, y, color);
    
    if (dx1 > dy1) {
        int err = dx1 >> 1;
        while (x != (int)x2) {
            err -= dy1;
            if (err < 0) {
                y += py;
                err += dx1;
            }
            x += px;
            display_set_pixel(x, y, color);
        }
    } else {
        int err = dy1 >> 1;
        while (y != (int)y2) {
            err -= dx1;
            if (err < 0) {
                x += px;
                err += dy1;
            }
            y += py;
            display_set_pixel(x, y, color);
        }
    }
    display_update();
}

void display_draw_rect(uint32_t x, uint32_t y, uint32_t width, uint32_t height, uint32_t color) {
    display_draw_line(x, y, x + width - 1, y, color);
    display_draw_line(x + width - 1, y, x + width - 1, y + height - 1, color);
    display_draw_line(x + width - 1, y + height - 1, x, y + height - 1, color);
    display_draw_line(x, y + height - 1, x, y, color);
}

void display_fill_rect(uint32_t x, uint32_t y, uint32_t width, uint32_t height, uint32_t color) {
    for (uint32_t i = y; i < y + height; i++) {
        for (uint32_t j = x; j < x + width; j++) {
            display_set_pixel(j, i, color);
        }
    }
    display_update();
} 