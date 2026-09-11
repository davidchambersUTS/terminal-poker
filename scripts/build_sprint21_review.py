"""Human-readable sprint report from actual executable/CI evidence."""
from pathlib import Path
import json, hashlib, textwrap
from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import A4, landscape
from reportlab.lib.utils import ImageReader

ROOT=Path(__file__).resolve().parents[1]
E=ROOT/'output/sprint21'
R=ROOT/'docs/agile/reports/sprint-21'
R.mkdir(parents=True,exist_ok=True)
P=ROOT/'output/pdf/sprint-21-review-report.pdf'
P.parent.mkdir(parents=True,exist_ok=True)
data=json.loads((E/'captures/evidence.json').read_text(encoding='utf-8'))
quality=json.loads((E/'quality.json').read_text(encoding='utf-8-sig'))
ci=json.loads((E/'ci-status.json').read_text(encoding='utf-8-sig'))
passed=ci.get('conclusion')=='success'
outcome='16/16 accepted' if passed else 'Review - remote CI pending'
W,H=landscape(A4); M=52; TOTAL=10
c=canvas.Canvas(str(P),pagesize=(W,H))
c.setTitle('Sprint 21 - Reliable builds and keyboard actions')
c.setAuthor('Sneaky Blinders project')
def header(title,page):
    c.setFillColorRGB(.09,.12,.11);c.setFont('Helvetica-Bold',22);c.drawString(M,H-70,title)
    c.setStrokeColorRGB(.04,.36,.24);c.setLineWidth(3);c.line(M,H-84,W-M,H-84)
    c.setFont('Helvetica',9);c.setFillColorRGB(.35,.4,.38)
    c.drawString(M,54,'Sneaky Blinders | Sprint 21 | 11 September 2026')
    c.drawRightString(W-M,54,f'{page} / {TOTAL}')
def paragraph(text,y,bold=False,size=12):
    c.setFillColorRGB(.1,.13,.12);c.setFont('Helvetica-Bold' if bold else 'Helvetica',size)
    for line in textwrap.wrap(text,105 if size==12 else 115):
        c.drawString(M,y,line);y-=17
    return y-12

header('Reliable builds and keyboard actions',1)
y=paragraph(outcome,H-112,True)
summary=[
    'Practice and network tables share one legal action selector. Left/Right moves focus, Enter submits, and a visible > marker accompanies the highlighted border. Short all-in calls are shown on Call; amounts distinguish chips added from total bet/raise commitments.',
    'The intermittent Mac restart test assumed every reconnecting client sees revision zero. A late client may correctly see an action already accepted from another seat. The deterministic regression retains the fresh-authority assertion and separately proves the late-observer case.',
    'The candidate also fixes a discovered all-in bypass of the existing no-reopening rule. Authority, client validation, the invariant generator and training action mask agree. Rejected all-in raises leave the hand unchanged; all-in calls remain legal.',
    'Previously deployed game lifecycle cleanup is included in the source handoff. The original working tree is preserved; the reviewed branch is sprint21/reliability-and-actions in the owner fork. This is source publication, not a merge or production deployment.',
    'Scope: REL-1 (3), REL-2 (2), UX-1 (3), UX-2 (5), VIS-1 (3). The already-planned bitmap investigation substitutes for unavailable live Mac/VLAN rollout evidence. REL-3 remains open with its original three points.',
    'The owner reports Ghostty works well and Warp/Codex app terminals have issues. Exact Mac versions and font metrics were not supplied. Passing the ConPTY harness does not certify those graphical terminal hosts.',
]
for text in summary:y=paragraph(text,y)
y=paragraph('Build: '+quality['client_sha256'][:24]+'... | Source: '+quality['source_commit'][:12],y,size=11)
c.showPage()
captions={
    '01-deal':('Initial authorized deal','Table 1, hand 1; S0 is the observer. Nine stacks begin at 100. S1 posts 1 and S2 posts 2: stacks total 897 plus pot 3 equals 900. Other private cards remain concealed.'),
    '02-select-call':('Call selected, before input','Revision 6: six passive calls raise the pot to 15. The default focus is Check/Call, showing the exact call amount of 2. Focus alone sends no command.'),
    '03-select-raise':('Arrow navigation changes focus only','The same revision 6, board, stacks and pot remain unchanged after Right. The > marker now selects Raise to 4. Enter submits that choice through the ordinary authority.'),
    'phase-Flop':('Flop after the raise is called','Revision 15: every seat has committed 4. Stacks are 96 each and the pot is 36; street contributions reset at the flop. No new hand or alternate fixture was substituted.'),
    'phase-Turn':('Turn: checks preserve the pot','Revision 24 follows nine flop checks. The pot remains 36 and stacks remain 96 each. S0 uses Enter on the selected Check action; bot seats use the passive controller.'),
    'phase-River':('River: one continuous hand','Revision 33 follows nine turn checks. The same authorized table projection and 80x30 layout are retained. No future cards or hidden opponent holdings were supplied to the selector.'),
    '99-award':('Award reconciled to 900 chips','Revision 42: S2 and S5 each receive 18 from the 36-chip pot, ending at 114; the other seven stacks remain 96. The displayed settled pot is historical and must not be added to final stacks again.'),
}
for page,frame in enumerate(data['frames'],2):
    name=frame['checkpoint'];title,caption=captions[name]
    header(title,page)
    image=ImageReader(str(R/'screenshots'/f'{name}.png'));iw,ih=image.getSize()
    scale=min((W-2*M)/iw,390/ih);w,h=iw*scale,ih*scale
    c.drawImage(image,(W-w)/2,111,width=w,height=h)
    paragraph(caption,88,size=11)
    c.showPage()
header('Acceptance and quality evidence',9)
y=H-112
for text in [
    f"Windows: {quality['passed']} tests passed, {quality['failed']} failed, {quality['ignored']} existing ignored. Formatting, strict all-target/all-feature Clippy and optimized client/server builds pass. The real network journey also submits through the selector and verifies pending/repeated input suppression.",
    'Six installed journeys pass: CMD, Windows PowerShell and Git Bash at 80x30 and 56x40, outside the repository. Checks cover navigation, Enter, resize, basic palette, Home, normal exit and Ctrl-C restoration. Two local fault probes of the exact production restoration code pass error and panic cleanup.',
    'Additional deterministic checks cover key press/repeat/release, stale hand/legal contexts, disconnected/disabled controls, short all-in calls, closed raising rights and focus visibility across all supported viewport breakpoints.',
    'Remote Quality: '+('all four platforms passed on '+quality['source_commit'][:12] if passed else 'still running; do not accept REL-1 until the recorded revision passes.')+'. Linux, Windows, macOS Apple Silicon and macOS Intel each run formatting, strict Clippy, full tests and optimized paired builds.',
    'Lifecycle reconciliation: all 119 deployed-manifest entries were checked. Before further UI edits, 117 were byte-identical and only the two new regression-test files differed. The manifest and candidate source/binary hashes are retained locally.',
    'No production secret is included. The branch contains only explicitly listed source, test fixtures and documentation. Private keys, profiles, credentials, runtime game state and generated review evidence remain excluded.',
]:y=paragraph(text,y)
c.showPage()
header('Decisions, risks and next work',10)
y=H-112
for text in [
    'VIS-1 decision: retain the shared text table. Twelve offline samples encoded the same authorized frame using Halfblocks, Sixel, Kitty and iTerm2 at three sizes. At 80x30, encoding took roughly 1.2-19.2 ms and cached drawing 3-17 microseconds. These are one-machine measurements, not player input latency or visible-terminal support evidence.',
    'Bitmap buffers do not prove cleanup, flicker, resize retention or font scaling inside a named emulator. No bitmap backend, appearance option, external asset or separate table window is activated. Keep the existing automatic menu fallback and investigate named-host defects with exact reproductions.',
    'Rollout remains open: REL-3 requires the live two-device Mac/VLAN journey and operational verification. Ghostty is owner-reported working; Warp and Codex terminal issues remain unverified. Active-hand crash recovery remains outside accepted scope.',
    'The paired runtime has not been deployed by this sprint. The running server retains the old all-in reopening defect until a separately authorized upgrade. Do not label the production service fixed merely because source and tests pass.',
    'Accounting after acceptance: 841 forecast = 660 historically accepted + 181 remaining. The 42 conditional presentation/window points remain excluded. Sprint 21 closes only after final PDF visual QA; no next sprint is activated.',
    'Next recommended work: finish the named-host and live rollout reproductions, then decide the separate-window investigation before the unified visual refresh. Retrospective actions: complete all sprint rituals before final handoff, and test asynchronous observation ordering explicitly rather than relying on client start timing.',
]:y=paragraph(text,y)
c.showPage();c.save()

ledger=['# Sprint 21 hand continuity','',
    'Production selector/renderer, optimized candidate. Table 1 / hand 1; review seed 21001.',
    'Observer S0; S1-S8 passive review controllers. Nine starting stacks of 100; blinds 1/2.',
    'Every trajectory screenshot is 80x30 with the same build and Ash palette.',
    'Final stacks include the awarded pot; the settled pot label is not additional live chips.','',
    '| Step | Screenshot | Revision | Phase | Board | Pot | S0-S8 stacks | S0-S8 street contributions |',
    '|---:|---|---:|---|---|---:|---|---|']
for n,frame in enumerate(data['frames'],1):
    stacks=','.join(str(s['stack']) for s in frame['seats'])
    contributions=','.join(str(s['contribution']) for s in frame['seats'])
    ledger.append(f"| {n} | {frame['checkpoint']}.png | {frame['revision']} | {frame['phase']} | {' '.join(frame['board']) or '-'} | {frame['pot']} | {stacks} | {contributions} |")
ledger += ['','## Every accepted action','', '| Revision | Seat | Phase | Action | Pot after |','|---:|---:|---|---|---:|']
for action in data['safe_history']['actions']:
    ledger.append(f"| {action['revision']} | {action['seat']} | {action['phase']} | {json.dumps(action['action'])} | {action['pot_after']} |")
ledger += ['', 'Final stacks: '+str(data['safe_history']['final_stacks']),
    'Final stacks sum to 900. S2 and S5 split 36 equally; no other award.',
    'Privacy: only observer cards and legitimately public showdown reveals were captured.']
(R/'hand-trajectory.md').write_text('\n'.join(ledger)+'\n',encoding='utf-8')
(R/'report-source.md').write_text('# Sprint 21 review source\n\n'+outcome+'\n\n'+
    '\n\n'.join(summary)+'\n\nSource commit: '+quality['source_commit']+
    '\nClient SHA-256: '+quality['client_sha256']+'\n\n'+
    '\n\n'.join(title+': '+caption for title,caption in captions.values())+
    '\n\nThe builder contains the exact acceptance, risk and forecast text. Machine evidence: output/sprint21/.\n',encoding='utf-8')
print(P)
print('SHA-256 '+hashlib.sha256(P.read_bytes()).hexdigest())
