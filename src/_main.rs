const W:usize=80;
const H:usize=24;
const D:f32=0.015707963;
const A:f32=4./3.*((H-2)as f32);
fn g(t:f32)->[f32;2]{[t.cos(),t.sin()]}
fn f(s:f32)->[f32;3]{[s,2.*s,3.*s]} fn h(k: usize)->f32{k as f32*D}
fn heart(x:[usize;2],t:usize)->((f32,f32,f32),f32){
    let (u,v,w)=(f(h(x[0])).map(g),g(h(x[1])),g(h(t)));
    let p=(v[1]*(15.*u[0][1]-4.*u[2][1]),8.*v[0],v[1]*(15.*u[0][0]-6.*u[1][0]-2.*u[2][0]));
    let du=(v[1]*(15.*u[0][0]-12.*u[2][0]),0.,v[1]*(-15.*u[0][1]+12.*u[1][1]+6.*u[2][1]));
    let dv=(v[0]*(15.*u[0][1]-4.*u[2][1]),-8.*v[1],v[0]*(15.*u[0][0]-6.*u[1][0]-2.*u[2][0]));
    let n=(-dv.1*du.2,du.2*dv.0-du.0*dv.2,du.0*dv.1);
    let nn=(n.0*n.0+n.1*n.1+n.2*n.2).sqrt();
    ((p.0*w[0]-p.1*w[1],p.0*w[1]+p.1*w[0],p.2),-((n.0*w[0]-n.1*w[1])*2.+(n.0*w[1]+n.1*w[0])*10.+n.2*6.)/nn)
}

fn screen(x:(f32,f32,f32))->usize{
    let x=(A*x.0/(28.+x.1),A*(x.2+3.)/(28.+x.1));
    let p=(((W as f32)/2.+x.0) as usize, ((H as f32-x.1)/2.) as usize);
    p.0 + W*p.1
} fn main() {
    println!("\x1b[?25l\x1b[1;31m");
    for m in 0..4000 { let mut bf=[' ';H*W]; let mut ybf=[0.;H*W];
        for k in 0..400 {
            for l in 0..200{
                let (q,z)=heart([k,l],m);
                let y=1./q.1;
                if z>=0.{
                    let p=screen(q);
                    if y>ybf[p]{
                        ybf[p]=y;bf[p]=".,-~:;=!*#$@".chars().nth(z as usize).unwrap();
                    }
                }
            }
        }
        print!("\x1b[H{}",(bf.chunks(W).map(String::from_iter)).collect::<Vec<_>>().join("\n"));
    }
    print!("\x1b[0m\x1b[?25h");
} //est2008//CCS//+Y