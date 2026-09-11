"""Exercise the installed candidate through real Windows ConPTY shell processes."""
from pathlib import Path
import os, sys, json, time, threading, queue, shutil, tempfile

ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'output/sprint21'
LAUNCH=Path(tempfile.gettempdir())/'sneakyblinders-sprint21-launch'
sys.path.insert(0,str(OUT/'python-deps'))
from winpty import PtyProcess
import pyte

def run(name,argv,width,height,basic=False):
    env=os.environ.copy()
    env['PATH']=str(OUT/'install')+os.pathsep+env['PATH']
    if basic: env['NO_COLOR']='1'
    process=PtyProcess.spawn(argv,cwd=str(LAUNCH),env=env,dimensions=(height,width))
    chunks=queue.Queue()
    raw=[]
    screen=pyte.Screen(width,height)
    stream=pyte.Stream(screen)
    def read():
        try:
            while True: chunks.put(process.read(65536))
        except (EOFError,OSError): pass
    threading.Thread(target=read,daemon=True).start()
    def pump(seconds=.1):
        until=time.monotonic()+seconds
        while time.monotonic()<until:
            try:
                text=chunks.get(timeout=.05)
                raw.append(text)
                stream.feed(text)
            except queue.Empty: pass
        return '\n'.join(screen.display)
    def wait_for(needle,seconds=15):
        until=time.monotonic()+seconds
        while time.monotonic()<until:
            text=pump()
            if needle.casefold() in text.casefold(): return text
            if not process.isalive(): break
        raise AssertionError(f'{name} missing {needle}: {text[-2000:]}')
    record={'shell':name,'viewport':[width,height],'host':'Windows ConPTY','basic_palette':basic,'launch_directory':str(LAUNCH),'exit_key':'Ctrl-C' if basic else 'q'}
    try:
        record['home']=wait_for('Quick Practice')
        process.write('\r')
        record['before']=wait_for('YOU TO ACT')
        process.write('\x1b[C')
        record['right']=wait_for('> R')
        process.write('\x1b[D')
        record['left']=wait_for('> C')
        process.write('\r')
        pump(.6)
        record['after_enter']='\n'.join(screen.display)
        assert 'panicked' not in ''.join(raw)
        process.setwinsize(40,56)
        screen.resize(lines=40,columns=56)
        record['resized']=pump(.8)
        assert 'Terminal too' not in record['resized']
        process.setwinsize(20,40)
        screen.resize(lines=20,columns=40)
        record['unsupported']=wait_for('Terminal too')
        process.write('\x1b')
        record['returned_home']=wait_for('Quick Practice')
        process.write('\x03' if basic else 'q')
        until=time.monotonic()+8
        while process.isalive() and time.monotonic()<until: pump()
        assert not process.isalive(),'candidate did not exit'
        pump(.2)
        record['clean_exit']=process.exitstatus==0
        assert record['clean_exit'],process.exitstatus
        record['alternate_screen_restored']='\x1b[?1049l' in ''.join(raw)
        assert record['alternate_screen_restored']
        record['pass']=True
    finally:
        (OUT/f'terminal-{name}-{width}x{height}.json').write_text(json.dumps(record,indent=2),encoding='utf-8')
        if process.isalive(): process.terminate(force=True)
    return {k:v for k,v in record.items() if k not in ['home','before','right','left','after_enter','resized','unsupported','returned_home']}

if __name__=='__main__':
    (OUT/'install').mkdir(exist_ok=True)
    assert not LAUNCH.resolve().is_relative_to(ROOT.resolve())
    LAUNCH.mkdir(exist_ok=True)
    shutil.copy2(ROOT/'target/release/sneakyblinders.exe',OUT/'install/sneakyblinders.exe')
    shells=[('cmd',['C:/Windows/System32/cmd.exe','/d','/c','sneakyblinders']),
        ('powershell',['C:/Windows/System32/WindowsPowerShell/v1.0/powershell.exe','-NoProfile','-Command','sneakyblinders']),
        ('git-bash',['C:/Program Files/Git/bin/bash.exe','--noprofile','--norc','-c','sneakyblinders'])]
    results=[]
    for name,argv in shells:
        for width,height in [(80,30),(56,40)]:
            result=run(name,argv,width,height,basic=width==56)
            results.append(result)
            print(json.dumps(result),flush=True)
    (OUT/'terminal-matrix.json').write_text(json.dumps(results,indent=2),encoding='utf-8')
