const {test, expect} = require('@playwright/test');
async function ready(page) {
 await page.goto('/');
 await expect(page.locator('#game-status')).toHaveAttribute('data-phase','0');
 await page.locator('canvas').focus();
}
async function status(page, field, value) {
 await expect(page.locator('#game-status')).toHaveAttribute('data-'+field,String(value));
}
function layout(w,h) {
 const cell=Math.min(Math.max(Math.min((w-40)/8,(h-260)/8),20),62);
 return {cell,ox:(w-cell*8)/2,oy:150,bw:Math.min((w-48)/3,180)};
}
async function square(page,x,y,button='left') {
 const {width:w,height:h}=page.viewportSize();const {cell,ox,oy}=layout(w,h);
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
 await square(page,4,0);await status(page,'phase',3);
 const lost=await page.locator('#game-status').textContent();
 await square(page,7,7,'right');await square(page,7,7);
 await expect(page.locator('#game-status')).toHaveText(lost);
 await page.keyboard.press('r');await status(page,'phase',1);
 await page.keyboard.press('n');await status(page,'phase',0);
 await page.screenshot({path:'test-results/desktop.png'});
});
test('reference board can be won and flags update automatically',async({page})=>{
 await page.setViewportSize({width:680,height:780});await ready(page);
 await page.mouse.click(524,89);await status(page,'phase',1);
 const mines=new Set(['4,0','4,1','5,2','7,2','1,3','7,3','7,4','1,5','6,5','0,6']);
 for(let y=0;y<8;y++)for(let x=0;x<8;x++)if(!mines.has(x+','+y))await square(page,x,y);
 await status(page,'phase',2);await status(page,'remaining',0);await status(page,'revealed',54);
});
test('mobile touch flag mode, reveal and reset',async({browser})=>{
 const context=await browser.newContext({viewport:{width:390,height:760},isMobile:true,hasTouch:true});
 const page=await context.newPage();await ready(page);
 const {cell,ox,oy}=layout(390,760);
 await page.touchscreen.tap(195,oy+cell*8+29); // visible flag-mode control
 await page.touchscreen.tap(ox+cell/2,oy+cell/2);await status(page,'remaining',9);await status(page,'phase',0);
 await page.touchscreen.tap(ox+cell/2,oy+cell/2);await status(page,'remaining',10);
 await page.touchscreen.tap(195,oy+cell*8+29);
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
