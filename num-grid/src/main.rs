use rand::seq::SliceRandom;
use rand::Rng;
use yew::prelude::*;

pub const NUMLENGTH: i64 = 30;
pub const NUMRANGE: i64 = 1000;

pub enum Msg {
    Randomize,
    ChangeNums,
}
pub struct Grid {
    nums: Vec<i64>,
}

impl Grid {
    fn update_nums(&mut self) {
        let mut rng = rand::thread_rng();

        let set: Vec<i64> = (0..NUMLENGTH)
            .map(|_| {
                let idx = rng.gen_range(0..NUMRANGE);
                idx
            })
            .collect();

        self.nums = set;
    }

    fn shuffle_nums(&mut self) {
        let mut rng = rand::thread_rng();
        self.nums.shuffle(&mut rng)
    }
}

impl Component for Grid {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        let mut rng = rand::thread_rng();

        let mut arr: Vec<i64> = (0..NUMLENGTH).collect();
        arr.shuffle(&mut rng);

        Self { nums: arr }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Randomize => {
                self.shuffle_nums();
            }

            Msg::ChangeNums => {
                self.update_nums();
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {

            <>
            <div class="grid">{


                self.nums.iter().map(|num|
                 {
                    html!{
                         <p class="box">{num}</p>
                    }
                }).collect::<Html>()
            }
           </div>

          <div class="btn-container">
          <button onclick={ctx.link().callback(|_| {
              Msg::Randomize

            })  } class="btn">{"Rearrange Numbers"}</button>

            <button onclick={ctx.link().callback(|_|Msg::ChangeNums) } class="btn">{"Change Numbers"}</button>

          </div>
         </>
        }
    }
}

fn main() {
    yew::start_app::<Grid>();
}
