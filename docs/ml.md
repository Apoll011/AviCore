# ml

```Namespace: global/ml```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> predict </h2>

```rust,ignore
fn predict(x: Array, model: Model) -> Array
```

<div>
<div class="tab">
<button group="predict" id="link-predict-Description"  class="tablinks active" 
    onclick="openTab(event, 'predict', 'Description')">
Description
</button>
</div>

<div group="predict" id="predict-Description" class="tabcontent"  style="display: block;" >
Uses a [`smartcore`](https://smartcorelib.org/) machine learning model (trained with the
[`train`](#trainx-array-y-array-algorithm-immutablestring---model) function to predict
dependent variables.
```typescript
let xdata = [[1.0, 2.0],
             [2.0, 3.0],
             [3.0, 4.0]];
let ydata = [1.0, 2.0, 3.0];
let model = train(xdata, ydata, "linear");
let ypred = predict(xdata, model);
true
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> train </h2>

```rust,ignore
fn train(x: Array, y: Array, algorithm: String) -> Model
```

<div>
<div class="tab">
<button group="train" id="link-train-Description"  class="tablinks active" 
    onclick="openTab(event, 'train', 'Description')">
Description
</button>
</div>

<div group="train" id="train-Description" class="tabcontent"  style="display: block;" >
Trains a [`smartcore`](https://smartcorelib.org/) machine learning model. The model can then
be used to make predictions with the [`predict`](#predictx-array-model-model---array)
function Available model types are:
1. `linear` - ordinary least squares linear regression
2. `logistic` - logistic regression
3. `lasso` - lasso regression
```typescript
let xdata = [[1.0, 2.0],
             [2.0, 3.0],
             [3.0, 4.0]];
let ydata = [1.0, 2.0, 3.0];
let model = train(xdata, ydata, "linear");
true;
```
</div>

</div>
</div>
</br>
