package com.example.lazydevs.lazydevsfinalproject;

import android.content.Intent;
import android.os.AsyncTask;
import android.util.Log;

import com.mashape.unirest.http.HttpResponse;
import com.mashape.unirest.http.JsonNode;
import com.mashape.unirest.http.Unirest;
import com.mashape.unirest.http.exceptions.UnirestException;

import org.json.JSONArray;
/**
 * Created by d95wang on 3/8/17.
 */

public class RecipeQuery extends AsyncTask {

    private int recipeID;
    private String url;
    private JSONArray jsonArray;
    private android.support.v4.app.FragmentManager fragmentManager;
    private Intent intent;
    private int fragID;

    public RecipeQuery(int id, android.support.v4.app.FragmentManager f, Intent i, int fragment){
        Log.d("Constructor", "in here!");
        this.recipeID = id;
        this.url = "https://spoonacular-recipe-food-nutrition-v1.p.mashape.com/recipes/";
        this.fragmentManager = f;
        this.intent = i;
        this.fragID = fragment;
    }

    public void setUrl(){
        this.url += recipeID;
        this.url += "/information?includeNutrition=false";
        Log.d("URL", this.url);
    }

    @Override
    protected Void doInBackground(Object[] params) {

        try{
            Log.d("TAG", "in here");
            HttpResponse<JsonNode> response = Unirest.get(url)
                    .header("X-Mashape-Key", "PU1nIf2t9qmshCbZ00eHoOcXwGWwp1xg73Ijsn2wUTB0udHndc")
                    .header("Accept", "application/json")
                    .asJson();
            this.jsonArray = response.getBody().getArray();
        } catch (UnirestException e) {
            e.printStackTrace();
        }
        return null;
    }



    @Override
    protected void onPostExecute(Object o) {
        super.onPostExecute(o);
        AppState.getInstance().setCurrentRecipe(jsonArray);
        RecipeFragment recipeFrag = new RecipeFragment();
        fragmentManager.beginTransaction().add(fragID, recipeFrag).commit();
    }
}
