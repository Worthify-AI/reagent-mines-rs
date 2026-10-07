const {test, expect} = require('@playwright/test');
async function ready(page) {
 await page.goto('/');
 await expect(page.locator('#game-status')).toHaveAttribute('data-phase','0');
 await page.locator('canvas').focus();
}
async function status(page, field, value) {
 await expect(page.locator('#game-status')).toHaveAttribute('data-'+field,String(value));
}
function layout(w,h,original=false) {
 const scale=original?1:Math.min(Math.max(Math.min((w-40)/220,(h-260)/258),1),2);
 const panelX=Math.round((w-220*scale)/2);
 return {cell:20*scale,ox:panelX+30*scale,oy:150+31*scale,bottom:150+258*scale+12,modeX:panelX+70*scale,bw:Math.min((w-48)/3,180)};
}
async function square(page,x,y,button='left',original=false) {
 const {width:w,height:h}=page.viewportSize();const {cell,ox,oy}=layout(w,h,original);
 await page.mouse.move(ox+(x+.5)*cell,oy+(y+.5)*cell);
 await page.mouse.down({button});await page.waitForTimeout(35);
 await page.mouse.up({button});await page.waitForTimeout(35);
}
test('desktop mouse, fixed-board comparison, loss, reset and keyboard', async ({page})=>{
 await page.setViewportSize({width:680,height:780});await ready(page);
 await square(page,0,0,'right');await status(page,'remaining',9);await status(page,'phase',0);
 await square(page,0,0);await status(page,'phase',0);
 await square(page,0,0,'right');await status(page,'remaining',10);
 await page.keyboard.press('Space');await status(page,'remaining',9);
 await page.keyboard.press('Space');await status(page,'remaining',10);
 await page.keyboard.press('Enter');await status(page,'phase',1);
 await expect(page.locator('#game-status')).not.toHaveAttribute('data-revealed','0');
 await page.keyboard.press('r');await status(page,'phase',0);await status(page,'remaining',10);
 await page.mouse.click(524,89);await status(page,'phase',1);
 await page.screenshot({path:'test-results/rust-reference-board.png'});
 await page.screenshot({path:'test-results/rust-reference-panel.png',clip:{x:230,y:150,width:220,height:258}});
 await page.screenshot({path:'test-results/rust-reference-cells.png',clip:{x:257,y:178,width:166,height:166}});
 await square(page,4,0,'left',true);await status(page,'phase',3);
 const lost=await page.locator('#game-status').textContent();
 await square(page,7,7,'right',true);await square(page,7,7,'left',true);
 await expect(page.locator('#game-status')).toHaveText(lost);
 await page.keyboard.press('r');await status(page,'phase',1);
 await page.keyboard.press('n');await status(page,'phase',0);
 await page.screenshot({path:'test-results/desktop.png'});
});
test('reference board can be won and flags update automatically',async({page})=>{
 await page.setViewportSize({width:680,height:780});await ready(page);
 await page.mouse.click(524,89);await status(page,'phase',1);
 const mines=new Set(['4,0','4,1','5,2','7,2','1,3','7,3','7,4','1,5','6,5','0,6']);
 for(let y=0;y<8;y++)for(let x=0;x<8;x++)if(!mines.has(x+','+y))await square(page,x,y,'left',true);
 await status(page,'phase',2);await status(page,'remaining',0);await status(page,'revealed',54);
});
test('mobile touch flag mode, reveal and reset',async({browser})=>{
 const context=await browser.newContext({viewport:{width:390,height:760},isMobile:true,hasTouch:true});
 const page=await context.newPage();await ready(page);
 const {cell,ox,oy,bottom,modeX}=layout(390,760);
 await page.touchscreen.tap(modeX,bottom+17); // visible flag-mode control
 await page.touchscreen.tap(ox+cell/2,oy+cell/2);await status(page,'remaining',9);await status(page,'phase',0);
 await page.touchscreen.tap(ox+cell/2,oy+cell/2);await status(page,'remaining',10);
 await page.touchscreen.tap(modeX,bottom+17);
 await page.touchscreen.tap(ox+cell/2,oy+cell/2);await status(page,'phase',1);
 await page.screenshot({path:'test-results/mobile.png'});
 await page.touchscreen.tap(195,89);await status(page,'phase',0);
 await context.close();
});
test('loader, wasm and page produce no errors',async({page})=>{
 const errors=[];page.on('pageerror',e=>errors.push(e.message));
 page.on('console',e=>{if(e.type()==='error')errors.push(e.text());});
 const failures=[];page.on('response',r=>{if(r.status()>=400)failures.push(r.url()+':'+r.status());});
 await ready(page);await page.waitForTimeout(1000);
 expect(errors).toEqual([]);expect(failures).toEqual([]);
 expect(await page.locator('canvas').evaluate(c=>c.width>0&&c.height>0)).toBe(true);
});

test('eight observed references cycle and flagged-empty replay uses real actions',async({page})=>{
 await page.setViewportSize({width:680,height:780});await ready(page);
 const traces=[...require('../../fixtures/calibration.json'),...require('../../fixtures/holdout.json')];
 const boards=require('../../fixtures/observed-boards.json');
 for(let i=0;i<boards.length;i++){
  if(i===0)await page.mouse.click(524,89);else await page.keyboard.press('b');
  const initial=traces.find(t=>t.board_id===boards[i].id).initial;
  const opened=initial.flat().filter(c=>/^[0-8]$/.test(c)).length;
  await status(page,'phase',1);await status(page,'remaining',10);await status(page,'revealed',opened);
  await page.screenshot({path:'test-results/rust-observed-board-'+(i+1)+'.png'});
 }
 await page.keyboard.press('b');await status(page,'revealed',12); // cycle wraps to board 1
 for(let step=1;step<=4;step++){
  await page.mouse.click(340,573); // visible step button at original panel size
  await status(page,'remaining',step<3?9:10);
  await status(page,'revealed',step===1?12:17);
  // Each frame's action is complete before advancing the next step.
  await page.waitForTimeout(40);
 }
 await page.screenshot({path:'test-results/rust-flagged-empty-hole.png'});
 await page.screenshot({path:'test-results/rust-flagged-empty-panel.png',clip:{x:230,y:150,width:220,height:258}});
 // The covered hole can still be revealed directly.
 await square(page,7,0,'left',true);await status(page,'revealed',18);
 await page.keyboard.press('r');await status(page,'revealed',12);
 for(let y=0;y<3;y++)for(let x=4;x<8;x++)await square(page,x,y,'right',true);
 await status(page,'remaining',-2);await status(page,'revealed',12);
 await page.screenshot({path:'test-results/rust-extra-flags.png'});
 await page.screenshot({path:'test-results/rust-extra-flags-panel.png',clip:{x:230,y:150,width:220,height:258}});
});
