// JavaScript representation of the fib function
function fibonacci(iter) {
    let local1 = 0;
    let local2 = 0;
    let local3 = 1;
    let local4 = 1;
  
    if (iter < 1) {
      return 0;
    }
  
    for (let i = 0; i < iter - 1; i++) {
      local1 = local3 + local4;
      local2 = local4;
      local3 = local4;
      local4 = local1;
    }
  
    return local2;
  }  
  
const function_name = "fib";

module.exports = {
    [function_name]: fibonacci
}
  