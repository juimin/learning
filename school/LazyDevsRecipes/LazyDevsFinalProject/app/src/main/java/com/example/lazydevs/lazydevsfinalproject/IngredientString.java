package com.example.lazydevs.lazydevsfinalproject;

/**
 * Created by victoriajuan on 3/7/17.
 */

class IngredientString {
    private static final IngredientString ourInstance = new IngredientString();
    public String ingredientString;


    static IngredientString getInstance() {
        return ourInstance;
    }

    public IngredientString() {
        this.ingredientString = null;
    }

}
