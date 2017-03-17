use std::ops::Deref;

const BYTES_PER_PAGE: u64 = 1 << 20; // = 1 mb
const MAX_NUM_PAGES: u64 = 1 << 44;

pub struct CPU {
    regs: [u64; 4],
    pages_root: PageTree,
}

struct Page {
    memory: [u8; BYTES_PER_PAGE as usize],
}

enum PageTree {
    Empty,
    Leaf(Box<Page>),
    Split(Box<PageTree>, Box<PageTree>),
}
use self::PageTree::*;

impl Page {
    fn new() -> Page {
        Page { memory: [0; BYTES_PER_PAGE as usize] }
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: [0; 4],
            pages_root: PageTree::Empty,
        }
    }

    pub fn reg_a(&self) -> &u64 { &self.regs[0] }
    pub fn reg_b(&self) -> &u64 { &self.regs[1] }
    pub fn reg_c(&self) -> &u64 { &self.regs[2] }
    pub fn reg_d(&self) -> &u64 { &self.regs[3] }

    pub fn load(&self, addr: u64) -> u8 {
        let offs = addr % BYTES_PER_PAGE;
        let pix = addr / BYTES_PER_PAGE;
        match self.pages_root.find_page(pix, 0, MAX_NUM_PAGES) {
            Some(pg) => pg.memory[offs as usize],
            None => 0,
        }
    }

    pub fn store(&mut self, addr: u64, val: u8) {
        let offs = addr % BYTES_PER_PAGE;
        let pix = addr / BYTES_PER_PAGE;
        let pg = self.pages_root.make_page(pix, 0, MAX_NUM_PAGES);
        pg.memory[offs as usize] = val;
    }
}

impl PageTree {
    fn find_page(&self, pix: u64, lo: u64, hi: u64) -> Option<&Page> {
        match self {
            &Empty => None,
            &Leaf(ref pg) => Some(pg.deref()),
            &Split(ref pl, ref pr) => {
                let mid = (lo + hi) / 2;
                if pix < mid {
                    pl.find_page(pix, lo, mid)
                }
                else {
                    pr.find_page(pix, mid, hi)
                }
            },
        }
    }

    fn make_page(&mut self, pix: u64, lo: u64, hi: u64) -> &mut Page {
        match self {
            &mut Leaf(ref mut pg) => pg,
            &mut Empty => {
                self.split(lo, hi);
                self.make_page(pix, lo, hi)
            },
            &mut Split(ref mut pl, ref mut pr) => {
                let mid = (lo + hi) / 2;
                if pix < mid {
                    pl.make_page(pix, lo, mid)
                }
                else {
                    pr.make_page(pix, mid, hi)
                }
            },
        }
    }

    fn split(&mut self, lo: u64, hi: u64) {
        if (hi - lo) > 1 {
            *self = Split(Box::new(Empty), Box::new(Empty));
        }
        else {
            *self = Leaf(Box::new(Page::new()));
        }
    }
}
