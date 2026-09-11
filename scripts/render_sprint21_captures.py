"""Render actual Ratatui cells, without mock terminal chrome or invented states."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont
from render_ui_concepts import terminal_color

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'output/sprint21/captures'
DEST = ROOT / 'docs/agile/reports/sprint-21/screenshots'
DEST.mkdir(parents=True, exist_ok=True)
font = ImageFont.truetype('C:/Windows/Fonts/DejaVuSansMono.ttf', 16)
bold = ImageFont.truetype('C:/Windows/Fonts/DejaVuSansMono-Bold.ttf', 16)
for source in SOURCE.glob('*.json'):
    data = json.loads(source.read_text(encoding="utf-8"))
    if 'cells' not in data:
        continue
    assert data['backend'] == 'ratatui::backend::TestBackend'
    width, height = data['width'], data['height']
    assert len(data['cells']) == width * height
    image = Image.new('RGB', (width*10,height*20),'black')
    draw = ImageDraw.Draw(image)
    for i, cell in enumerate(data['cells']):
        x,y = (i%width)*10,(i//width)*20
        fg = terminal_color(cell['foreground'],'#e2e2e2')
        bg = terminal_color(cell['background'],'#000000')
        mods = cell['modifiers']
        if mods & 64: fg,bg = bg,fg
        draw.rectangle((x,y,x+9,y+19),fill=bg)
        if cell['symbol'] and not mods & 128:
            draw.text((x,y),cell['symbol'],font=bold if mods&1 else font,fill=fg)
    image.save(DEST/(source.stem+'.png'))
print(DEST)
