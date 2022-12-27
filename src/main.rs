extern crate rand;
extern crate sfml;

use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture};
use sfml::window::{Event, Style, mouse::Button, Key};
use sfml::graphics::Rect;
use sfml::graphics::Transformable;
use sfml::system::Vector2f;
use rand::Rng;

/*
    TODO: Fix bomb not triggering loss when quick click is used
    TODO: Explore odd bugs (clicked tile is unexplored, yet other tiles surrounding it is explored, seems to be for v=0)
    TODO: Add win status
    TODO: Add Clock
    TODO: Add Highscore list
*/

fn main() {
    let mut window = RenderWindow::new(
        (1200, 800),
        "Annars Minesweeper",
        Style::CLOSE,
        &Default::default(),
    );

    let xsize = 30;
    let ysize = 20;
    let tile_sizex = 21;
    let tile_sizey = 21;

    let texture = match Texture::from_file("assets/m_tileset.png")
    {
        Some(tex) => tex,
        None => {
                    println!("Unable to load tilesheet!");
                    return;
                },
    };

    let mut v = vec![vec![0u32; ysize]; xsize];
    let mut explored = vec![vec![false; ysize]; xsize];
    let mut flags = vec![vec![false; ysize]; xsize];
    let mut sprites = vec![Sprite::with_texture(&texture); ysize*xsize];

    let max_bombs = xsize*ysize/6;

    generate_board(&mut v, (xsize, ysize), max_bombs);

    let zero     =  Rect::new(             0,  0,   tile_sizex, tile_sizey);
    let one      =  Rect::new(tile_sizex * 1,  0,   tile_sizex, tile_sizey);
    let two      =  Rect::new(tile_sizex * 2,  0,   tile_sizex, tile_sizey);
    let three    =  Rect::new(tile_sizex * 3,  0,   tile_sizex, tile_sizey);
    let four     =  Rect::new(tile_sizex * 4,  0,   tile_sizex, tile_sizey);
    let five     =  Rect::new(tile_sizex * 5,  0,   tile_sizex, tile_sizey);
    let six      =  Rect::new(tile_sizex * 6,  0,   tile_sizex, tile_sizey);
    let seven    =  Rect::new(tile_sizex * 7,  0,   tile_sizex, tile_sizey);
    let eight    =  Rect::new(tile_sizex * 8,  0,   tile_sizex, tile_sizey);
    let flag     =  Rect::new(tile_sizex * 9,  0,   tile_sizex, tile_sizey);
    let clock    =  Rect::new(tile_sizex * 10, 0,   tile_sizex, tile_sizey);
    let mine     =  Rect::new(tile_sizex * 11, 0,   tile_sizex, tile_sizey);
    let unknown  =  Rect::new(tile_sizex * 12, 0,   tile_sizex, tile_sizey);
    let hit_mine =  Rect::new(tile_sizex * 13, 0,   tile_sizex, tile_sizey);

    let mut update_sprites = true;
    let mut lost = false;
    let mut hit = (0, 0);

    loop
    {
        while let Some(e) = window.poll_event() {
            match e {
                Event::Closed => return,
                Event::MouseButtonPressed {button, x, y} => {
                    if x < xsize as i32 * tile_sizex && y < ysize as i32 * tile_sizey  
                    {
                        let x_i = (x / tile_sizex) as usize;
                        let y_i = (y / tile_sizey) as usize;

                        if button == Button::Left && v[x_i][y_i] == count_adjacent_flags(&flags, (xsize, ysize), (x_i, y_i))
                        {
                            quick_click(&v, &mut explored, &flags, (x_i, y_i), (xsize, ysize));
                        } 
                        else if button == Button::Left && v[x_i][y_i] == 9 && !flags[x_i][y_i]
                        {
                            lost = true;
                            hit = (x_i, y_i);
                        }
                        else if button == Button::Right && !explored[x_i][y_i]
                        {
                            flags[x_i][y_i] = !flags[x_i][y_i];    
                        }
                        else if button == Button::Left && explored[x_i][y_i] == false 
                        {
                            explore(&v, &mut explored, &flags, x_i, y_i, xsize, ysize);
                        }
                        
                        update_sprites = true;
                    } 
                }
                Event::KeyPressed {code, alt, ctrl, shift, system} => {
                    if code == Key::Escape
                    {
                        return;
                    }
                    else if code == Key::E
                    {
                        zero_board(&mut v, &mut explored, &mut flags, (xsize, ysize));
                        generate_board(&mut v, (xsize, ysize), max_bombs);
                        update_sprites = true;
                        lost = false;
                    }
                }
                _ => {}
            }
        }
        if update_sprites || (lost && update_sprites)
        {
            for x in 0..xsize
            {
                for y in 0..ysize
                {
                    let x_ = x as f32 * tile_sizex as f32;
                    let y_ = y as f32 * tile_sizey as f32;

                    let pos = Vector2f::new(x_, y_);
                    sprites[xsize * y + x].set_position(pos);
                    //println!("Position of tile: ({},{})", x_, y_);
                    if flags[x][y]
                    {
                        sprites[xsize * y + x].set_texture_rect(&flag);
                    }
                    else if explored[x][y] || lost
                    {
                        match v[x][y] 
                        {
                            0 => sprites[xsize * y + x].set_texture_rect(&zero),
                            1 => sprites[xsize * y + x].set_texture_rect(&one),
                            2 => sprites[xsize * y + x].set_texture_rect(&two),
                            3 => sprites[xsize * y + x].set_texture_rect(&three),
                            4 => sprites[xsize * y + x].set_texture_rect(&four),
                            5 => sprites[xsize * y + x].set_texture_rect(&five),
                            6 => sprites[xsize * y + x].set_texture_rect(&six),
                            7 => sprites[xsize * y + x].set_texture_rect(&seven),
                            8 => sprites[xsize * y + x].set_texture_rect(&eight),
                            9 => {
                                if hit.0 == x && hit.1 == y
                                {
                                    sprites[xsize * y + x].set_texture_rect(&hit_mine);    
                                }
                                else
                                {
                                    sprites[xsize * y + x].set_texture_rect(&mine);
                                }
                            }
                            _ => sprites[xsize * y + x].set_texture_rect(&unknown),
                        };
                    }
                    else 
                    {
                        sprites[xsize * y + x].set_texture_rect(&unknown);
                    }
                }
            }
            update_sprites = false;
        
        }

        window.clear(&Color::WHITE);

        for i in 0..xsize*ysize
        {
            window.draw(&sprites[i]);            
        }

        window.display();
    }
}

fn quick_click(v: &Vec<Vec<u32>>, e: &mut Vec<Vec<bool>>, flags: &Vec<Vec<bool>>, pos: (usize, usize), sz: (usize, usize))
{
    let (x_i, y_i) = pos;
    let (xsize, ysize) = sz;

    if x_i != 0 && y_i != 0 
    {   explore(v, e, flags, x_i-1, y_i-1, xsize, ysize); }
    if x_i != 0
    {   explore(v, e, flags, x_i-1, y_i  , xsize, ysize); }
    if x_i != 0 && y_i != ysize - 1
    {   explore(v, e, flags, x_i-1, y_i+1, xsize, ysize); }
    if y_i != 0
    {   explore(v, e, flags, x_i  , y_i-1, xsize, ysize); }
    if y_i != ysize - 1 
    {   explore(v, e, flags, x_i  , y_i+1, xsize, ysize); }
    if x_i != xsize - 1 && y_i != 0 
    {   explore(v, e, flags, x_i+1, y_i-1, xsize, ysize); }
    if x_i != xsize - 1
    {   explore(v, e, flags, x_i+1, y_i  , xsize, ysize); }
    if x_i != xsize - 1 && y_i != ysize - 1
    {   explore(v, e, flags, x_i+1, y_i+1, xsize, ysize); }
}

fn count_adjacent_flags(flags: &Vec<Vec<bool>>, sz: (usize, usize), pos: (usize, usize)) -> u32
{
    let (xsize, ysize) = sz;
    let (x_pos, y_pos) = pos;

    let mut number_of_flags = 0;

    if x_pos != 0 && y_pos != 0 && flags[x_pos - 1][y_pos - 1]
    {
        number_of_flags = number_of_flags + 1;
    }
    if x_pos != 0 && flags[x_pos - 1][y_pos]
    {
        number_of_flags = number_of_flags + 1;
    }
    if x_pos != 0 && y_pos != ysize - 1 && flags[x_pos - 1][y_pos + 1]
    {
        number_of_flags = number_of_flags + 1;
    }
    if y_pos != 0 && flags[x_pos][y_pos - 1]
    {
        number_of_flags = number_of_flags + 1;
    }
    if y_pos != ysize - 1 && flags[x_pos][y_pos + 1]
    {
        number_of_flags = number_of_flags + 1;
    }
    if x_pos != xsize - 1 && y_pos != 0 && flags[x_pos + 1][y_pos - 1]
    {
        number_of_flags = number_of_flags + 1;
    }
    if x_pos != xsize - 1 && flags[x_pos + 1][y_pos]
    {
        number_of_flags = number_of_flags + 1;
    }
    if x_pos != xsize - 1 && y_pos != ysize - 1 && flags[x_pos + 1][y_pos + 1]
    {
        number_of_flags = number_of_flags + 1;
    }

    return number_of_flags;
}

fn zero_board(v: &mut Vec<Vec<u32>>, e: &mut Vec<Vec<bool>>, f: &mut Vec<Vec<bool>>, sz: (usize, usize))
{
    let (xsize, ysize) = sz;

    for x in 0..xsize as usize
    {
        for y in 0..ysize as usize
        {
            v[x][y] = 0;
            e[x][y] = false;
            f[x][y] = false;
        }
    }    
}

fn generate_board(v: &mut Vec<Vec<u32>>, sz: (usize, usize), max_bombs: usize)
{
    let (xsize, ysize) = sz;

    let mut generated_bombs = 0;

    while generated_bombs < max_bombs
    {
        let x = rand::thread_rng().gen_range(0, xsize);
        let y = rand::thread_rng().gen_range(0, ysize);

        if v[x][y] != 9 
        {
            generated_bombs = generated_bombs + 1;
            v[x][y] = 9;
        }
    }

    for i in 0..xsize as usize
    {
        for j in 0..ysize as usize
        {
            if v[i][j] != 9
            {
                let mut bombs = 0;
                if i != 0 && j != 0 && v[i - 1][j - 1] == 9
                {
                    bombs = bombs + 1;
                }
                if i != 0 && v[i - 1][j] == 9
                {
                    bombs = bombs + 1;
                }
                if i != 0 && j != ysize - 1 && v[i - 1][j + 1] == 9
                {
                    bombs = bombs + 1;
                }
                if j != 0 && v[i][j - 1] == 9
                {
                    bombs = bombs + 1;
                }
                if j != ysize - 1 && v[i][j + 1] == 9
                {
                    bombs = bombs + 1;
                }
                if i != xsize - 1 && j != 0 && v[i + 1][j - 1] == 9
                {
                    bombs = bombs + 1;
                }
                if i != xsize - 1 && v[i + 1][j] == 9
                {
                    bombs = bombs + 1;
                }
                if i != xsize - 1 && j != ysize - 1 && v[i + 1][j + 1] == 9
                {
                    bombs = bombs + 1;
                }

                v[i][j] = bombs;
            }
        }
    }
}

fn explore(v: &Vec<Vec<u32>>, e: &mut Vec<Vec<bool>>, f: &Vec<Vec<bool>>, x: usize, y: usize, xsize: usize, ysize: usize)
{
    //base case
    if e[x][y] || f[x][y] 
    {
        return;
    }
    
    e[x][y] = true;
    
    if v[x][y] == 0 
    {
        if x != 0 && y != 0                 {explore(v, e, f, x - 1, y - 1, xsize, ysize);}
        if x != 0                           {explore(v, e, f, x - 1, y    , xsize, ysize);}
        if x != 0 && y != ysize - 1         {explore(v, e, f, x - 1, y + 1, xsize, ysize);}
        if y != 0                           {explore(v, e, f, x    , y - 1, xsize, ysize);}
        if y != ysize - 1                   {explore(v, e, f, x    , y + 1, xsize, ysize);}
        if x != xsize - 1 && y != 0         {explore(v, e, f, x + 1, y - 1, xsize, ysize);}
        if x != xsize - 1                   {explore(v, e, f, x + 1, y    , xsize, ysize);}
        if x != xsize - 1 && y != ysize - 1 {explore(v, e, f, x + 1, y + 1, xsize, ysize);}
    }
}
