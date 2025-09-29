// Global functions that can be called from Rust
globalThis.greet = function() {
    console.log("Hello from JavaScript!");
    return "Hello from JS!";
};

globalThis.greetPerson = function(name) {
    console.log("greetPerson called with:", name, typeof name);
    const message = "Hello, " + name + "!";
    console.log(message);
    return message;
};

globalThis.add = function(a, b) {
    console.log("add called with:", a, typeof a, b, typeof b);
    const result = a + b;
    console.log(a + " + " + b + " = " + result);
    return result;
};

globalThis.processData = function(name, age, isActive) {
    console.log("processData called with:", name, age, isActive);
    const data = {
        name: name,
        age: age,
        isActive: isActive,
        processed: true,
        timestamp: new Date().toISOString()
    };
    console.log("Processed data:", JSON.stringify(data, null, 2));
    return data;
};

globalThis.analyzeObject = function(obj) {
    console.log("analyzeObject called with:", obj);
    const keys = Object.keys(obj);
    const result = {
        keyCount: keys.length,
        keys: keys,
        hasName: 'name' in obj,
        originalData: obj
    };
    console.log("Analysis result:", JSON.stringify(result, null, 2));
    return result;
};

globalThis.sumArray = function(numbers) {
    console.log("sumArray called with:", numbers, Array.isArray(numbers));
    if (!Array.isArray(numbers)) {
        throw new Error("Input must be an array, got: " + typeof numbers);
    }
    const sum = numbers.reduce((acc, num) => acc + num, 0);
    console.log("Sum of [" + numbers.join(', ') + "] = " + sum);
    return sum;
};

globalThis.testHostFunctions = function() {
    console.log("Testing host functions...");
    const results = {};
    
    try {
        console.log("Calling hostAdd(10, 20)");
        results.hostAdd = rustyscript.functions.hostAdd(10, 20);
        console.log("hostAdd result:", results.hostAdd);
    } catch (e) {
        console.log("hostAdd error:", e);
        results.hostAdd = "error: " + e.message;
    }
    
    try {
        console.log("Calling hostGreet('JavaScript')");
        results.hostGreet = rustyscript.functions.hostGreet("JavaScript");
        console.log("hostGreet result:", results.hostGreet);
    } catch (e) {
        console.log("hostGreet error:", e);
        results.hostGreet = "error: " + e.message;
    }
    
    try {
        console.log("Calling hostGetTime()");
        results.hostGetTime = rustyscript.functions.hostGetTime();
        console.log("hostGetTime result:", results.hostGetTime);
    } catch (e) {
        console.log("hostGetTime error:", e);
        results.hostGetTime = "error: " + e.message;
    }
    
    return results;
};

globalThis.load = function() {
    console.log("Plugin loaded successfully!");
    return "loaded";
};

// Initialize the plugin
console.log("JavaScript plugin initialized with functions: greet, greetPerson, add, processData, analyzeObject, sumArray, callHost, load");