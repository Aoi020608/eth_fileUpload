pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let ret_vec = Vec::new(); 

    

    let mut nums_len = nums.len();

    for num in nums {
        ret_vec.push(num)
    }

    for i in nums.len() {
        for j in nums_len() {
           if nums[i] == (target - nums[j]) {
                
           } 
        } 

        nums_len -= 1;
    }

    ret_vec
    
}
